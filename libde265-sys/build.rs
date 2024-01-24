use const_format::formatcp;
use curl::easy::Easy;
use flate2::read::GzDecoder;
use std::{
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
};
use tar::Archive;
use walkdir::WalkDir;

const LIBDE265_VERSION: &str = "1.0.15";
const LIBDE265_NAME: &str = formatcp!("libde265-{LIBDE265_VERSION}");
const LIBDE265_FILE_NAME: &str = formatcp!("{LIBDE265_NAME}.tar.gz");
const LIBDE265_URL: &str = formatcp!(
    "https://github.com/strukturag/libde265/releases/download/v{LIBDE265_VERSION}/{LIBDE265_FILE_NAME}",
);

fn download<P: AsRef<Path>>(source_url: &str, target_file: P) -> anyhow::Result<()> {
    let f = fs::File::create(&target_file)?;
    let mut writer = io::BufWriter::new(f);
    let mut easy = Easy::new();
    easy.useragent("Curl Download")?;
    easy.url(source_url)?;
    easy.follow_location(true)?;
    easy.write_function(move |data| Ok(writer.write(data).unwrap()))?;
    easy.perform()?;

    let response_code = easy.response_code()?;
    if response_code == 200 {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Unexpected response code {} for {}",
            response_code,
            source_url
        ))
    }
}

fn extract<P1: AsRef<Path>, P2: AsRef<Path>>(filename: P1, outpath: P2) -> anyhow::Result<()> {
    let file = fs::File::open(&filename)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    archive.unpack(outpath.as_ref())?;

    Ok(())
}

/// Finds all files with an extension, ignoring some.
fn glob_import<P: AsRef<Path>>(root: P, extenstion: &str, exclude: &[&str]) -> Vec<String> {
    WalkDir::new(root)
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| x.path().to_str().unwrap().ends_with(extenstion))
        .map(|x| x.path().to_str().unwrap().to_string())
        .filter(|x| !exclude.iter().any(|e| x.contains(e)))
        .collect()
}

fn feature_enabled(feature: &str) -> bool {
    let env_var_name = format!("CARGO_FEATURE_{}", feature.replace('-', "_").to_uppercase());
    println!("cargo:rerun-if-env-changed={env_var_name}");
    env::var(env_var_name).is_ok()
}

fn compile_and_add_libde265_static_lib(root: &Path, libname: &str, encoder: bool) {
    let libde265_src = root.join("libde265");
    let mut cc_build = cc::Build::new();
    let files = glob_import(
        &libde265_src,
        ".cc",
        if encoder {
            &[]
        } else {
            &["encoder", "en265.cc"]
        },
    );

    cc_build
        .include(&root)
        .include(&libde265_src)
        .cpp(true)
        .warnings(false)
        .define("HAVE_MALLOC_H", "true")
        .files(files)
        .flag_if_supported("-msse4.1")
        .flag_if_supported("-mavx2")
        .pic(true);

    cc_build.compile(libname);

    println!("cargo:rustc-link-lib=static={libname}");
}

fn generate_bindings(root: &Path) -> anyhow::Result<()> {
    let builder = bindgen::Builder::default()
        .header(format!("{}", root.join("libde265/en265.h").display()))
        .allowlist_type("(de|en)265_.*")
        .allowlist_item("(de|en)265_.*")
        .clang_arg("-std=c++14")
        .clang_arg("-x")
        .clang_arg("c++")
        .constified_enum_module(".*")
        .layout_tests(false);

    builder.generate()?.write_to_file("./src/ffi.rs")?;

    Ok(())
}

fn build_libde265_from_sources() -> anyhow::Result<()> {
    let encoder = feature_enabled("encoder");
    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    let libname = format!("libde265{}", if encoder { "_en" } else { "" });

    let lib_path = out_path.join(format!("{libname}.a"));
    if !lib_path.exists() {
        let archive_file = out_path.join(LIBDE265_FILE_NAME);
        let archive_root_dir = out_path.join(LIBDE265_NAME);

        if !archive_root_dir.exists() {
            download(LIBDE265_URL, &archive_file)?;
            extract(archive_file, &out_path)?;
        }

        compile_and_add_libde265_static_lib(&archive_root_dir, &libname, encoder);

        if feature_enabled("generate-bindings") {
            generate_bindings(&archive_root_dir)?;
        }
    }

    Ok(())
}

fn link_system_libde265() -> anyhow::Result<()> {
    println!("cargo:rustc-link-lib=dylib=de265");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    if feature_enabled("static") {
        build_libde265_from_sources()?;
    } else if feature_enabled("system") {
        link_system_libde265()?;
    } else {
        panic!("Either `system` or `static` feature should be enabled!");
    }

    Ok(())
}

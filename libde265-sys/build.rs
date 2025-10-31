fn feature_enabled(feature: &str) -> bool {
    let env_var_name = format!("CARGO_FEATURE_{}", feature.replace('-', "_").to_uppercase());
    println!("cargo:rerun-if-env-changed={env_var_name}");
    std::env::var(env_var_name).is_ok()
}

fn main() {
    if feature_enabled("system") {
        println!("cargo:rustc-link-lib=dylib=de265");
    } else if !feature_enabled("static") {
        panic!("Either `system` or `static` feature should be enabled!");
    }
}

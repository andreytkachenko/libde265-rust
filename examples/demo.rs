use libde265::{De265, Decoder};

const NAL_MIN_0_COUNT: usize = 2;
#[inline]
fn nth_nal_index(stream: &[u8], nth: usize) -> Option<usize> {
    let mut count_0 = 0;
    let mut n = 0;

    for (i, byte) in stream.iter().enumerate() {
        match byte {
            0 => count_0 += 1,
            1 if count_0 >= NAL_MIN_0_COUNT => {
                if n == nth {
                    return Some(i - NAL_MIN_0_COUNT);
                } else {
                    count_0 = 0;
                    n += 1;
                }
            }
            _ => count_0 = 0,
        }
    }

    None
}

pub fn nal_units(mut stream: &[u8]) -> impl Iterator<Item = &[u8]> {
    std::iter::from_fn(move || {
        let first = nth_nal_index(stream, 0);
        let next = nth_nal_index(stream, 1);

        match (first, next) {
            (Some(f), Some(n)) => {
                let rval = &stream[f..n];
                stream = &stream[n..];
                Some(rval)
            }
            (Some(f), None) => {
                let rval = &stream[f..];
                stream = &stream[f + NAL_MIN_0_COUNT..];
                Some(rval)
            }
            _ => None,
        }
    })
}

fn main() {
    let de = De265::new().unwrap();
    println!("libde265 {}", de.get_version());

    let mut decoder = Decoder::new(de.clone());
    decoder.start_worker_threads(16).unwrap();

    let h264_in = std::fs::read("/home/andrey/demo/h265/big_buck_bunney.h265").unwrap();

    for packet in nal_units(&h264_in) {
        decoder.push_data(packet, 0, None).unwrap();
        decoder.decode().unwrap();

        if let Some(img) = decoder.get_next_picture() {
            let y = img.get_image_plane(0);
            let u = img.get_image_plane(1);
            let v = img.get_image_plane(2);

            println!(
                "pic {}x{} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                img.get_image_width(0),
                img.get_image_height(0),
                img.get_chroma_format(),
                &y.0[0..20],
                y.1,
                &u.0[0..20],
                u.1,
                &v.0[0..20],
                v.1
            );
        }
    }
}

mod common;
use common::hash;

use std::{fs::File, io::BufReader};
use vtf::builder::VTFBuilder;

#[test]
fn test_animated() {
    let first_frame = image::load(
        BufReader::new(File::open("tests/data/rust_rgb_8.png").unwrap()),
        image::ImageFormat::Png,
    )
    .unwrap();

    let second_frame = image::load(
        BufReader::new(File::open("tests/data/rust_rgb_8_alpha.png").unwrap()),
        image::ImageFormat::Png,
    )
    .unwrap();

    let first_frame_hash = hash(&first_frame);
    let second_frame_hash = hash(&second_frame);

    let vtf = VTFBuilder::new(vtf::ImageFormat::Rgba8888)
        .add_frame(first_frame)
        .unwrap()
        .add_frame(second_frame)
        .unwrap()
        .set_first_frame(0)
        .build()
        .unwrap();

    let vtf = vtf::from_bytes(&vtf).unwrap();
    let decoded_first = vtf.highres_image.decode(0).unwrap();
    let decoded_second = vtf.highres_image.decode(1).unwrap();

    assert_eq!(first_frame_hash, hash(&decoded_first));
    assert_eq!(second_frame_hash, hash(&decoded_second));
}

use image::{open, GenericImageView};
use std::fs::File;
use std::io::Read;
use std::vec::Vec;

mod common;
use common::*;

fn test_image(input: &str, expected: &str) {
    let mut file = File::open(input).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let vtf = vtf::from_bytes(&buf).unwrap();
    let image = vtf.highres_image.decode(0).unwrap();

    let expected = open(expected).unwrap();

    assert_eq!(expected.dimensions(), image.dimensions());

    assert_eq!(hash(&expected), hash(&image));
}

#[test]
fn test_to_png_dxt5() {
    test_image("tests/data/rust_dxt5.vtf", "tests/data/rust_dxt5.png");
}

#[test]
fn test_to_png_rgb8() {
    test_image("tests/data/rust_rgb_8.vtf", "tests/data/rust_rgb_8.png");
}

#[test]
fn test_to_png_rgb8_alpha() {
    test_image(
        "tests/data/rust_rgb_8_alpha.vtf",
        "tests/data/rust_rgb_8_alpha.png",
    );
}

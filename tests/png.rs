use image::{open, GenericImageView, RgbaImage};
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::vec::Vec;

#[test]
fn test_to_png() {
    let mut file = File::open("tests/data/rust.vtf").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let vtf = vtf::from_bytes(&mut buf).unwrap();
    let image = vtf.highres_image.decode(0).unwrap();

    let expected = open("tests/data/rust.png").unwrap();

    assert_eq!(expected.dimensions(), image.dimensions());

    assert_eq!(
        hash(expected.as_rgba8().unwrap()),
        hash(image.as_rgba8().unwrap())
    );
}

fn hash(image: &RgbaImage) -> u64 {
    let mut hasher = DefaultHasher::new();
    image.hash(&mut hasher);
    hasher.finish()
}

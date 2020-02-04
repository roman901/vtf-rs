use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;

#[test]
fn test_info() {
    let mut file = File::open("tests/data/rust.vtf").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let vtf = vtf::from_bytes(&mut buf).unwrap();

    assert_eq!(256, vtf.header.width());
    assert_eq!(256, vtf.header.height());
    assert_eq!([7,3], vtf.header.version());
    assert_eq!(1, vtf.header.frames());
}

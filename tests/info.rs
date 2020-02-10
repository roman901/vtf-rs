use std::fs::File;
use std::io::Read;

use std::vec::Vec;

#[test]
fn test_info() {
    let mut file = File::open("tests/data/rust_dxt5.vtf").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let vtf = vtf::from_bytes(&mut buf).unwrap();

    assert_eq!(512, vtf.header.width);
    assert_eq!(512, vtf.header.height);
    assert_eq!(1, vtf.header.frames);
}

#[test]
fn test_write_header() {
    let mut file = File::open("tests/data/vtf_74.vtf").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let vtf = vtf::from_bytes(&mut buf).unwrap();

    let mut written = Vec::new();
    vtf.header.write(&mut written).unwrap();
    assert_eq!(written.as_slice(), &buf[0..written.len()]);
}

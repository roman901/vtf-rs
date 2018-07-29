mod vtf;
mod structs;

use vtf::VTF;
use std::io::{Error};

pub fn from_bytes(bytes: &mut Vec<u8>) -> Result<VTF, Error> {
    VTF::read(bytes)
}
mod header;
mod image;
mod utils;
mod vtf;

use std::io::Error;
use vtf::VTF;

pub fn from_bytes(bytes: &mut Vec<u8>) -> Result<VTF, Error> {
    VTF::read(bytes)
}

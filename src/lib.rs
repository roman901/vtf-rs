mod header;
mod image;
mod utils;
mod vtf;

use err_derive::Error;
use crate::vtf::VTF;

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "IO error: {}", _0)]
    Io(#[error(source)] std::io::Error),
    #[error(display = "File does not have a valid vtf signature")]
    InvalidSignature,
    #[error(display = "File does not have a valid vtf image format: {}", _0)]
    InvalidImageFormat(i16),
    #[error(display = "Error manipulating image data: {}", _0)]
    Image(#[error(source)] ::image::ImageError)
}

pub fn from_bytes(bytes: &mut Vec<u8>) -> Result<VTF, Error> {
    VTF::read(bytes)
}

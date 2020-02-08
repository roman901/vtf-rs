mod header;
mod image;
mod resources;
mod utils;
mod vtf;

use crate::image::ImageFormat;
use crate::vtf::VTF;
use ::image::DynamicImage;
use err_derive::Error;
use num_enum::TryFromPrimitiveError;

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "IO error: {}", _0)]
    Io(#[error(source)] std::io::Error),
    #[error(display = "File does not have a valid vtf signature")]
    InvalidSignature,
    #[error(display = "File does not have a valid vtf image format: {}", _0)]
    InvalidImageFormat(i16),
    #[error(display = "Error manipulating image data: {}", _0)]
    Image(#[error(source)] ::image::ImageError),
    #[error(display = "Decoding {} images is not supported", _0)]
    UnsupportedImageFormat(ImageFormat),
    #[error(display = "Decoded image data does not have the expected size")]
    InvalidImageData,
    #[error(display = "Image size needs to be a power of 2 and below 2^16")]
    InvalidImageSize,
}

impl From<TryFromPrimitiveError<image::ImageFormat>> for Error {
    fn from(err: TryFromPrimitiveError<image::ImageFormat>) -> Self {
        Error::InvalidImageFormat(err.number)
    }
}

pub fn from_bytes(bytes: &mut Vec<u8>) -> Result<VTF, Error> {
    VTF::read(bytes)
}

pub fn create(image: DynamicImage) -> Result<Vec<u8>, Error> {
    VTF::create(image)
}

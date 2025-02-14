pub mod header;
pub mod image;
pub mod resources;
mod utils;
pub mod vtf;

pub use crate::image::ImageFormat;
use crate::vtf::VTF;
use ::image::DynamicImage;
use num_enum::TryFromPrimitiveError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("File does not have a valid vtf signature")]
    InvalidSignature,
    #[error("File does not have a valid vtf image format: {0}")]
    InvalidImageFormat(i16),
    #[error("Error manipulating image data: {0}")]
    Image(#[from] ::image::ImageError),
    #[error("Decoding {0} images is not supported")]
    UnsupportedImageFormat(ImageFormat),
    #[error("Decoded image data does not have the expected size")]
    InvalidImageData,
    #[error("Image size needs to be a power of 2 and below 2^16")]
    InvalidImageSize,
    #[error("Encoding {0} images is not supported")]
    UnsupportedEncodeImageFormat(ImageFormat),
}

impl From<TryFromPrimitiveError<image::ImageFormat>> for Error {
    fn from(err: TryFromPrimitiveError<image::ImageFormat>) -> Self {
        Error::InvalidImageFormat(err.number)
    }
}

pub fn from_bytes(bytes: &[u8]) -> Result<VTF, Error> {
    VTF::read(bytes)
}

pub fn create(image: DynamicImage, image_format: ImageFormat) -> Result<Vec<u8>, Error> {
    VTF::create(image, image_format)
}

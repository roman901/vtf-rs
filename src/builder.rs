use crate::{Error, ImageFormat, vtf::VTF};
use image::{DynamicImage, GenericImageView};

#[derive(Clone, Debug)]
pub struct VTFBuilder {
    frames: Vec<DynamicImage>,
    image_format: ImageFormat,
    first_frame: u16,
}
impl VTFBuilder {
    pub fn new(image_format: ImageFormat) -> Self {
        VTFBuilder { frames: Vec::new(), image_format, first_frame: 0 }
    }

    pub fn add_frame(
        mut self,
        image: DynamicImage,
    ) -> Result<Self, Error> {
        if !image.width().is_power_of_two()
            || !image.height().is_power_of_two()
            || image.width() > u16::MAX as u32
            || image.height() > u16::MAX as u32
        {
            return Err(Error::InvalidImageSize);
        }

        if let Some(first) = self.frames.first() {
            if image.dimensions() != first.dimensions() {
                return Err(Error::MismatchedFrameDimensions);
            }
        }

        self.frames.push(image);

        Ok(self)
    }

    pub fn set_first_frame(mut self, first_frame: u16) -> Self {
        self.first_frame = first_frame;
        self
    }

    pub fn build(self) -> Result<Vec<u8>, Error> {
        if self.frames.is_empty() {
            return Err(Error::NoFrames);
        }

        if self.first_frame >= self.frames.len() as u16 {
            return Err(Error::InvalidFirstFrame);
        }

        VTF::encode(&self.frames, self.image_format, self.first_frame)
    }
}

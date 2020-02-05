use crate::header::VTFHeader;
use crate::utils::get_offset;
use crate::Error;
use image::dxt::{DXTDecoder, DXTVariant};
use image::{DynamicImage, ImageBuffer, ImageDecoder, Pixel};
use num_enum::TryFromPrimitive;
use parse_display::Display;
use std::ops::Deref;
use std::vec::Vec;

#[derive(Debug)]
pub struct VTFImage<'a> {
    pub header: VTFHeader,
    pub format: ImageFormat,
    pub width: u16,
    pub height: u16,
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> VTFImage<'a> {
    pub fn new(
        header: VTFHeader,
        format: ImageFormat,
        width: u16,
        height: u16,
        bytes: &'a Vec<u8>,
        offset: usize,
    ) -> VTFImage<'a> {
        VTFImage {
            header,
            format,
            width,
            height,
            bytes,
            offset,
        }
    }

    pub fn get_frame(&self, frame: u32) -> &[u8] {
        let frame_size = self
            .format
            .frame_size(self.width as u32, self.height as u32) as usize;
        let base: usize =
            self.offset + get_offset(&self.header, &self.format, frame, 0, 0, 0) as usize;
        &self.bytes[base..base + frame_size]
    }

    fn decode_dxt(&self, bytes: &[u8], variant: DXTVariant) -> Result<Vec<u8>, Error> {
        Ok(DXTDecoder::new(bytes, self.width as u32, self.height as u32, variant)?.read_image()?)
    }

    fn image_from_buffer<P, Container, F>(
        &self,
        buffer: Container,
        format: F,
    ) -> Result<DynamicImage, Error>
    where
        P: Pixel + 'static,
        P::Subpixel: 'static,
        Container: Deref<Target = [P::Subpixel]>,
        F: FnOnce(ImageBuffer<P, Container>) -> DynamicImage,
    {
        ImageBuffer::from_raw(self.width as u32, self.height as u32, buffer)
            .map(format)
            .ok_or(Error::NoDecoder(self.format))
    }

    pub fn decode(&self, frame: u32) -> Result<DynamicImage, Error> {
        let bytes = self.get_frame(frame);
        match self.format {
            ImageFormat::Dxt1 => {
                let buf = self.decode_dxt(bytes, DXTVariant::DXT1)?;
                self.image_from_buffer(buf, DynamicImage::ImageRgb8)
            }
            ImageFormat::Dxt1Onebitalpha => {
                let buf = self.decode_dxt(bytes, DXTVariant::DXT1)?;
                self.image_from_buffer(buf, DynamicImage::ImageRgba8)
            }
            ImageFormat::Dxt3 => {
                let buf = self.decode_dxt(bytes, DXTVariant::DXT3)?;
                self.image_from_buffer(buf, DynamicImage::ImageRgba8)
            }
            ImageFormat::Dxt5 => {
                let buf = self.decode_dxt(bytes, DXTVariant::DXT5)?;
                self.image_from_buffer(buf, DynamicImage::ImageRgba8)
            }
            ImageFormat::Rgba8888 => {
                self.image_from_buffer(bytes.to_vec(), DynamicImage::ImageRgba8)
            }
            ImageFormat::Rgb888 => self.image_from_buffer(bytes.to_vec(), DynamicImage::ImageRgb8),
            ImageFormat::Bgr888 => self.image_from_buffer(bytes.to_vec(), DynamicImage::ImageBgr8),
            ImageFormat::Bgra8888 => {
                self.image_from_buffer(bytes.to_vec(), DynamicImage::ImageBgra8)
            }
            _ => Err(Error::NoDecoder(self.format)),
        }
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, TryFromPrimitive)]
#[repr(i16)]
pub enum ImageFormat {
    None = -1,
    Rgba8888 = 0,
    Abgr8888,
    Rgb888,
    Bgr888,
    Rgb565,
    I8,
    Ia88,
    P8,
    A8,
    Rgb888Bluescreen,
    Bgr888Bluescreen,
    Argb8888,
    Bgra8888,
    Dxt1,
    Dxt3,
    Dxt5,
    Bgrx8888,
    Bgr565,
    Bgrx5551,
    Bgra4444,
    Dxt1Onebitalpha,
    Bgra5551,
    Uv88,
    Uvwq8888,
    Rgba16161616f,
    Rgba16161616,
    Uvlx8888,
}

impl ImageFormat {
    pub fn frame_size(&self, width: u32, height: u32) -> u32 {
        match self {
            ImageFormat::None => 0,
            ImageFormat::Rgba8888 => width * height * 4,
            ImageFormat::Abgr8888 => width * height * 4,
            ImageFormat::Rgb888 => width * height * 3,
            ImageFormat::Bgr888 => width * height * 3,
            ImageFormat::Rgb565 => width * height * 2,
            ImageFormat::I8 => width * height * 1,
            ImageFormat::Ia88 => width * height * 2,
            ImageFormat::A8 => width * height,
            ImageFormat::Argb8888 => width * height * 4,
            ImageFormat::Bgra8888 => width * height * 4,
            ImageFormat::Dxt1 => ((width + 3) / 4) * ((height + 3) / 4) * 8,
            ImageFormat::Dxt5 => ((width + 3) / 4) * ((height + 3) / 4) * 16,
            ImageFormat::Rgba16161616f => width * height * 8,
            ImageFormat::Rgba16161616 => width * height * 8,
            _ => panic!("ImageFormat {:?} not supported", self),
        }
    }
}

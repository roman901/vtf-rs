use crate::header::VTFHeader;
use crate::utils::get_offset;

use std::vec::Vec;

#[derive(Debug)]
pub struct VTFImage<'a> {
    pub header: VTFHeader,
    pub format: ImageFormat,
    pub width: u16,
    pub height: u16,
    bytes: &'a Vec<u8>,
}

impl<'a> VTFImage<'a> {
    pub fn new(
        header: VTFHeader,
        format: ImageFormat,
        width: u16,
        height: u16,
        bytes: &'a Vec<u8>,
    ) -> VTFImage<'a> {
        VTFImage {
            header,
            format,
            width,
            height,
            bytes,
        }
    }

    pub fn get_frame(&self, frame: u32) -> Vec<u8> {
        let frame_size = self
            .format
            .frame_size(self.width as u32, self.height as u32) as usize;
        let fulldata = get_offset(&self.header, &self.format, 0, 0, 0, -1) as usize;
        let base: usize = self.bytes.len() - fulldata
            + get_offset(&self.header, &self.format, frame, 0, 0, 0) as usize;
        self.bytes[base..base + frame_size].to_vec()
    }
}

#[derive(Debug)]
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
    pub fn from(num: i16) -> ImageFormat {
        match num {
            -1 => ImageFormat::None,
            0 => ImageFormat::Rgba8888,
            13 => ImageFormat::Dxt1,
            14 => ImageFormat::Dxt3,
            15 => ImageFormat::Dxt5,
            _ => panic!("ImageFormat {} not supported", num),
        }
    }

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

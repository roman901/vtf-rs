use crate::header::VTFHeader;
use crate::image::{ImageFormat, VTFImage};
use crate::header::VTFHeaderBytes;

use std::io::{Cursor, Error, Read};
use std::vec::Vec;

const VTF_SIGNATURE: u32 = 0x00465456;

#[derive(Debug)]
pub struct VTF<'a> {
    pub header: VTFHeader,
    pub lowres_image: VTFImage<'a>,
    pub highres_image: VTFImage<'a>,
}

impl<'a> VTF<'a> {
    pub fn read(bytes: &mut Vec<u8>) -> Result<VTF, Error> {
        let mut cursor = Cursor::new(&bytes);

        let mut header = VTFHeaderBytes::new();

        cursor.read_exact(header.as_mut_bytes())?;
        let mut header = header.into_header();

        let signature = header.signature;
        assert_eq!(
            signature, VTF_SIGNATURE,
            "Specified data is not VTF file"
        );

        if header.version[0] < 7 || (header.version[0] == 7 && header.version[1] < 2) {
            header.depth = 1;
        }

        if header.version[0] < 7 || (header.version[0] == 7 && header.version[1] < 3) {
            header.num_resources = 0;
        }

        let lowres_image = VTFImage::new(
            header,
            ImageFormat::from(header.lowres_image_format as i16),
            header.lowres_image_width as u16,
            header.lowres_image_height as u16,
            bytes,
        );

        let highres_image = VTFImage::new(
            header,
            ImageFormat::from(header.highres_image_format as i16),
            header.width,
            header.height,
            bytes,
        );

        Ok(VTF {
            header,
            lowres_image,
            highres_image,
        })
    }
}

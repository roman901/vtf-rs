use crate::header::VTFHeader;
use crate::header::VTFHeaderBytes;
use crate::image::{ImageFormat, VTFImage};

use crate::Error;
use std::convert::TryFrom;
use std::io::{Cursor, Read};
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

        if header.signature() != VTF_SIGNATURE {
            return Err(Error::InvalidSignature);
        }

        let version = header.version();
        if version[0] < 7 || (version[0] == 7 && version[1] < 2) {
            header.set_depth(1);
        }

        if version[0] < 7 || (version[0] == 7 && version[1] < 3) {
            header.set_num_resources(0);
        }

        let lowres_image = VTFImage::new(
            header,
            ImageFormat::try_from(header.lowres_image_format() as i16)?,
            header.lowres_image_width() as u16,
            header.lowres_image_height() as u16,
            bytes,
        );

        let highres_image = VTFImage::new(
            header,
            ImageFormat::try_from(header.highres_image_format() as i16)?,
            header.width(),
            header.height(),
            bytes,
        );

        Ok(VTF {
            header,
            lowres_image,
            highres_image,
        })
    }
}

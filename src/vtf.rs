use crate::header::VTFHeader;
use crate::image::{ImageFormat, VTFImage};

use crate::resources::ResourceType;
use crate::Error;
use std::convert::TryFrom;
use std::io::Cursor;
use std::vec::Vec;

#[derive(Debug)]
pub struct VTF<'a> {
    pub header: VTFHeader,
    pub lowres_image: VTFImage<'a>,
    pub highres_image: VTFImage<'a>,
}

impl<'a> VTF<'a> {
    pub fn read(bytes: &mut Vec<u8>) -> Result<VTF, Error> {
        let mut cursor = Cursor::new(&bytes);

        let header = VTFHeader::read(&mut cursor)?;

        let lowres_format = ImageFormat::try_from(header.lowres_image_format as i16)?;
        let highres_format = ImageFormat::try_from(header.highres_image_format as i16)?;

        let lowres_offset = match header
            .resources
            .get_by_type(ResourceType::VTF_LEGACY_RSRC_LOW_RES_IMAGE)
        {
            Some(resource) => resource.data,
            None => header.header_size,
        };

        let highres_offset = match header
            .resources
            .get_by_type(ResourceType::VTF_LEGACY_RSRC_LOW_RES_IMAGE)
        {
            Some(resource) => resource.data,
            None => {
                lowres_offset
                    + lowres_format.frame_size(
                        header.lowres_image_width as u32,
                        header.lowres_image_height as u32,
                    )
            }
        };

        let lowres_image = VTFImage::new(
            header.clone(),
            lowres_format,
            header.lowres_image_width as u16,
            header.lowres_image_height as u16,
            bytes,
            lowres_offset as usize,
        );

        let highres_image = VTFImage::new(
            header.clone(),
            highres_format,
            header.width,
            header.height,
            bytes,
            highres_offset as usize,
        );

        Ok(VTF {
            header,
            lowres_image,
            highres_image,
        })
    }
}

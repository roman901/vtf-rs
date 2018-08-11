use header::VTFHeader;
use image::{VTFImage, ImageFormat};

use std::mem;
use std::slice;
use std::io::{Read, Cursor, Error};
use std::vec::Vec;
use std::iter::Iterator;

const VTF_SIGNATURE: u32 = 0x00465456;

#[derive(Debug)]
pub struct VTF<'a> {
    pub header: VTFHeader,
    pub lowres_image: VTFImage<'a>,
    pub highres_image: VTFImage<'a>
}

impl<'a> VTF<'a> {
    pub fn read(bytes: &mut Vec<u8>) -> Result<VTF, Error> {
        let mut cursor = Cursor::new(&bytes);

        let mut header: VTFHeader = unsafe { mem::uninitialized() };
        unsafe {
            let dst_ptr = &mut header as *mut VTFHeader as *mut u8;
            let slice = slice::from_raw_parts_mut(dst_ptr, mem::size_of::<VTFHeader>());

            cursor.read_exact(slice)?;
        }

        assert_eq!(header.signature, VTF_SIGNATURE, "Specified data is not VTF file");

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
            bytes
        );

        let highres_image = VTFImage::new(
            header,
            ImageFormat::from(header.highres_image_format as i16),
            header.width,
            header.height,
            bytes
        );

        Ok(VTF {
            header,
            lowres_image,
            highres_image
        })
    }
}
use structs::*;
use std::mem;
use std::slice;
use std::io::{Read, Cursor, Error};
use std::vec::Vec;

const VTF_SIGNATURE: u32 = 0x00465456;

#[derive(Debug)]
pub struct VTF {
    pub header: VTFHeader,
    pub lowres_image: VTFImage,
    pub highres_image: VTFImage
}

impl VTF {
    pub fn read(bytes: &mut Vec<u8>) -> Result<VTF, Error> {
        let mut cursor = Cursor::new(bytes);

        let mut header: VTFHeader = unsafe { mem::uninitialized() };
        unsafe {
            let dst_ptr = &mut header as *mut VTFHeader as *mut u8;
            let slice = slice::from_raw_parts_mut(dst_ptr, mem::size_of::<VTFHeader>());

            cursor.read_exact(slice)?;
        }

        assert_eq!(header.signature, VTF_SIGNATURE, "Specified data is not VTF file");

        //let image_format = header.highres_image_format as ImageFormat;
        let image_f = ImageFormat::from(header.highres_image_format);
        let lowres_image = VTFImage {
            format: ImageFormat::from(header.lowres_image_format),
            width: header.lowres_image_width as u16,
            height: header.lowres_image_height as u16
        };
        let highres_image = VTFImage {
            format: ImageFormat::from(header.highres_image_format),
            width: header.width,
            height: header.heigth
        };

        Ok(VTF {
            header,
            lowres_image,
            highres_image
        })
    }
}
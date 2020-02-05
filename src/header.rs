use crate::resources::ResourceList;
use crate::Error;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

#[derive(Debug, Clone)]
pub struct VTFHeader {
    pub signature: u32,
    pub version: [u32; 2],
    pub header_size: u32,
    pub width: u16,
    pub height: u16,
    pub flags: u32,
    pub frames: u16,
    pub first_frame: u16,
    pub reflectivity: [f32; 3],
    pub bumpmap_scale: f32,
    pub highres_image_format: u32,
    pub mipmap_count: u8,
    pub lowres_image_format: u32,
    pub lowres_image_width: u8,
    pub lowres_image_height: u8,
    pub depth: u16,
    pub resources: ResourceList,
}

const VTF_SIGNATURE: u32 = 0x00465456;

impl VTFHeader {
    pub fn read(bytes: &mut impl Read) -> Result<Self, Error> {
        let signature = bytes.read_u32::<LittleEndian>()?;

        if signature != VTF_SIGNATURE {
            return Err(Error::InvalidSignature);
        }

        let version = [
            bytes.read_u32::<LittleEndian>()?,
            bytes.read_u32::<LittleEndian>()?,
        ];
        let header_size = bytes.read_u32::<LittleEndian>()?;
        let width = bytes.read_u16::<LittleEndian>()?;
        let height = bytes.read_u16::<LittleEndian>()?;
        let flags = bytes.read_u32::<LittleEndian>()?;
        let frames = bytes.read_u16::<LittleEndian>()?;
        let first_frame = bytes.read_u16::<LittleEndian>()?;
        let _padding = bytes.read_u32::<LittleEndian>()?;
        let reflectivity = [
            bytes.read_f32::<LittleEndian>()?,
            bytes.read_f32::<LittleEndian>()?,
            bytes.read_f32::<LittleEndian>()?,
        ];
        let _padding = bytes.read_u32::<LittleEndian>()?;
        let bumpmap_scale = bytes.read_f32::<LittleEndian>()?;
        let highres_image_format = bytes.read_u32::<LittleEndian>()?;
        let mipmap_count = bytes.read_u8()?;
        let lowres_image_format = bytes.read_u32::<LittleEndian>()?;
        let lowres_image_width = bytes.read_u8()?;
        let lowres_image_height = bytes.read_u8()?;

        let depth = if version[0] >= 7 && version[1] >= 2 {
            bytes.read_u16::<LittleEndian>()?
        } else {
            1
        };
        let resources = if version[0] >= 7 && version[1] >= 3 {
            let _padding = [bytes.read_u8()?, bytes.read_u8()?, bytes.read_u8()?];
            let num_resources = bytes.read_u32::<LittleEndian>()?;
            ResourceList::read(bytes, num_resources)?
        } else {
            ResourceList::empty()
        };

        Ok(VTFHeader {
            signature,
            version,
            header_size,
            width,
            height,
            flags,
            frames,
            first_frame,
            reflectivity,
            bumpmap_scale,
            highres_image_format,
            mipmap_count,
            lowres_image_format,
            lowres_image_width,
            lowres_image_height,
            depth,
            resources,
        })
    }
}

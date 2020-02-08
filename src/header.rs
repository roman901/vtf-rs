use crate::image::ImageFormat;
use crate::resources::ResourceList;
use crate::Error;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::convert::TryFrom;
use std::io::{Read, Write};

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
    pub highres_image_format: ImageFormat,
    pub mipmap_count: u8,
    pub lowres_image_format: ImageFormat,
    pub lowres_image_width: u8,
    pub lowres_image_height: u8,
    pub depth: u16,
    pub resources: ResourceList,
}

impl VTFHeader {
    pub const SIGNATURE: u32 = 0x00465456;

    pub fn read(bytes: &mut impl Read) -> Result<Self, Error> {
        let signature = bytes.read_u32::<LittleEndian>()?;

        if signature != Self::SIGNATURE {
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
            highres_image_format: ImageFormat::try_from(highres_image_format as i16)?,
            mipmap_count,
            lowres_image_format: ImageFormat::try_from(lowres_image_format as i16)?,
            lowres_image_width,
            lowres_image_height,
            depth,
            resources,
        })
    }

    pub fn write(&self, bytes: &mut impl Write) -> Result<(), Error> {
        bytes.write_u32::<LittleEndian>(self.signature)?;

        bytes.write_u32::<LittleEndian>(self.version[0])?;
        bytes.write_u32::<LittleEndian>(self.version[1])?;

        bytes.write_u32::<LittleEndian>(self.size() as u32)?;
        bytes.write_u16::<LittleEndian>(self.width)?;
        bytes.write_u16::<LittleEndian>(self.height)?;
        bytes.write_u32::<LittleEndian>(self.flags)?;
        bytes.write_u16::<LittleEndian>(self.frames)?;
        bytes.write_u16::<LittleEndian>(self.first_frame)?;

        bytes.write_u32::<LittleEndian>(0)?;

        bytes.write_f32::<LittleEndian>(self.reflectivity[0])?;
        bytes.write_f32::<LittleEndian>(self.reflectivity[1])?;
        bytes.write_f32::<LittleEndian>(self.reflectivity[2])?;

        bytes.write_u32::<LittleEndian>(0)?;

        bytes.write_f32::<LittleEndian>(self.bumpmap_scale)?;
        bytes.write_u32::<LittleEndian>(self.highres_image_format as i16 as u32)?;
        bytes.write_u8(self.mipmap_count)?;
        bytes.write_u32::<LittleEndian>(self.lowres_image_format as i16 as u32)?;
        bytes.write_u8(self.lowres_image_width)?;
        bytes.write_u8(self.lowres_image_height)?;

        if self.version[0] >= 7 && self.version[1] >= 2 {
            bytes.write_u16::<LittleEndian>(self.depth)?;
        }

        if self.version[0] >= 7 && self.version[1] >= 3 {
            bytes.write_u8(0)?;
            bytes.write_u8(0)?;
            bytes.write_u8(0)?;
            bytes.write_u32::<LittleEndian>(self.resources.resources.len() as u32)?;
            bytes.write_u64::<LittleEndian>(0)?;
            self.resources.write(bytes)?;
        }

        Ok(())
    }

    pub fn size(&self) -> usize {
        match self.version[1] {
            0 | 1 => 64,
            _ => 80 + (self.resources.resources.len() * 8),
        }
    }
}

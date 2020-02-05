use crate::Error;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResourceType {
    id: [u8; 3],
    flags: u8,
}

impl ResourceType {
    pub const VTF_LEGACY_RSRC_LOW_RES_IMAGE: ResourceType = ResourceType {
        id: [0x01, 0x00, 0x00],
        flags: 0,
    };
    pub const VTF_LEGACY_RSRC_IMAGE: ResourceType = ResourceType {
        id: [0x30, 0x00, 0x00],
        flags: 0,
    };

    const HAS_NO_DATA_CHUNK: u8 = 0x02;

    pub fn has_resource_type(&self) -> bool {
        self.flags & Self::HAS_NO_DATA_CHUNK == 0
    }
}

impl ResourceType {
    pub fn read(bytes: &mut impl Read) -> Result<Self, Error> {
        Ok(ResourceType {
            id: [bytes.read_u8()?, bytes.read_u8()?, bytes.read_u8()?],
            flags: bytes.read_u8()?,
        })
    }

    pub fn write(&self, bytes: &mut impl Write) -> Result<(), Error> {
        bytes.write_u8(self.id[0])?;
        bytes.write_u8(self.id[1])?;
        bytes.write_u8(self.id[2])?;
        bytes.write_u8(self.flags)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Resource {
    ty: ResourceType,
    pub data: u32, // crc or offset
}

impl Resource {
    pub fn read(bytes: &mut impl Read) -> Result<Self, Error> {
        Ok(Resource {
            ty: ResourceType::read(bytes)?,
            data: bytes.read_u32::<LittleEndian>()?,
        })
    }

    pub fn write(&self, bytes: &mut impl Write) -> Result<(), Error> {
        self.ty.write(bytes)?;
        bytes.write_u32::<LittleEndian>(self.data)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ResourceList {
    pub resources: Vec<Resource>,
}

impl ResourceList {
    pub fn empty() -> Self {
        ResourceList {
            resources: Vec::new(),
        }
    }

    pub fn read(bytes: &mut impl Read, num_resources: u32) -> Result<Self, Error> {
        let _padding = bytes.read_u64::<LittleEndian>()?;

        let resources = (0..num_resources)
            .map(|_| Resource::read(bytes))
            .collect::<Result<Vec<Resource>, Error>>()?;
        Ok(ResourceList { resources })
    }

    pub fn write(&self, bytes: &mut impl Write) -> Result<(), Error> {
        self.resources
            .iter()
            .map(|resource| resource.write(bytes))
            .collect::<Result<(), Error>>()
    }

    pub fn get_by_type(&self, ty: ResourceType) -> Option<&Resource> {
        self.resources.iter().find(|resource| resource.ty == ty)
    }
}

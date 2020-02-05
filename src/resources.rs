use crate::Error;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

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
}

impl ResourceType {
    pub fn read(bytes: &mut impl Read) -> Result<Self, Error> {
        Ok(ResourceType {
            id: [bytes.read_u8()?, bytes.read_u8()?, bytes.read_u8()?],
            flags: bytes.read_u8()?,
        })
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
}

#[derive(Debug, Clone)]
pub struct ResourceList {
    resources: Vec<Resource>,
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

    pub fn get_by_type(&self, ty: ResourceType) -> Option<&Resource> {
        self.resources.iter().find(|resource| resource.ty == ty)
    }
}

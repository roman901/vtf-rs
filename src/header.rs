use std::mem::size_of;

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct VTFHeader {
    signature: u32,
    version: [u32; 2],
    header_size: u32,
    width: u16,
    height: u16,
    flags: u32,
    frames: u16,
    first_frame: u16,
    padding0: [u8; 4],
    reflectivity: [f32; 3],
    padding1: [u8; 4],
    bumpmap_scale: f32,
    highres_image_format: u32,
    mipmap_count: u8,
    lowres_image_format: u32,
    lowres_image_width: u8,
    lowres_image_height: u8,
    depth: u16,
    padding2: [u8; 3],
    num_resources: u32,
}

impl VTFHeader {
    pub fn signature(&self) -> u32 {
        self.signature
    }
    pub fn version(&self) -> [u32; 2] {
        self.version
    }
    pub fn header_size(&self) -> u32 {
        self.header_size
    }
    pub fn width(&self) -> u16 {
        self.width
    }
    pub fn height(&self) -> u16 {
        self.height
    }
    pub fn flags(&self) -> u32 {
        self.flags
    }
    pub fn frames(&self) -> u16 {
        self.frames
    }
    pub fn first_frame(&self) -> u16 {
        self.first_frame
    }
    pub fn padding0(&self) -> [u8; 4] {
        self.padding0
    }
    pub fn reflectivity(&self) -> [f32; 3] {
        self.reflectivity
    }
    pub fn padding1(&self) -> [u8; 4] {
        self.padding1
    }
    pub fn bumpmap_scale(&self) -> f32 {
        self.bumpmap_scale
    }
    pub fn highres_image_format(&self) -> u32 {
        self.highres_image_format
    }
    pub fn mipmap_count(&self) -> u8 {
        self.mipmap_count
    }
    pub fn lowres_image_format(&self) -> u32 {
        self.lowres_image_format
    }
    pub fn lowres_image_width(&self) -> u8 {
        self.lowres_image_width
    }
    pub fn lowres_image_height(&self) -> u8 {
        self.lowres_image_height
    }
    pub fn depth(&self) -> u16 {
        self.depth
    }
    pub fn padding2(&self) -> [u8; 3] {
        self.padding2
    }
    pub fn num_resources(&self) -> u32 {
        self.num_resources
    }
    pub fn set_depth(&mut self, depth: u16) {
        self.depth = depth;
    }
    pub fn set_num_resources(&mut self, num_resources: u32) {
        self.num_resources = num_resources;
    }
}

#[repr(C)]
pub union VTFHeaderBytes {
    header: VTFHeader,
    bytes: [u8; size_of::<VTFHeader>()],
}

impl VTFHeaderBytes {
    pub fn new() -> Self {
        VTFHeaderBytes { bytes: [0; size_of::<VTFHeader>()] }
    }

    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        unsafe { &mut self.bytes }
    }

    pub fn into_header(self) -> VTFHeader {
        // Safety: this is safe because all possible bit combinations are a valid VTFHeader
        unsafe { self.header }
    }
}
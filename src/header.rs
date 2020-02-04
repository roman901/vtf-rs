#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct VTFHeader {
    pub signature: u32,
    pub version: [u32; 2],
    pub header_size: u32,
    pub width: u16,
    pub height: u16,
    pub flags: u32,
    pub frames: u16,
    pub first_frame: u16,
    pub padding0: [u8; 4],
    pub reflectivity: [f32; 3],
    pub padding1: [u8; 4],
    pub bumpmap_scale: f32,
    pub highres_image_format: u32,
    pub mipmap_count: u8,
    pub lowres_image_format: u32,
    pub lowres_image_width: u8,
    pub lowres_image_height: u8,
    pub depth: u16,
    pub padding2: [u8; 3],
    pub num_resources: u32,
}

#[derive(Debug)]
#[repr(packed)]
pub struct VTFHeader {
    pub signature: u32,
    pub version: [u32; 2],
    pub header_size: u32,
    pub width: u16,
    pub heigth: u16,
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
    pub num_resources: u32
}

#[derive(Debug)]
pub struct VTFImage {
    pub format: ImageFormat,
    pub width: u16,
    pub height: u16
}

#[derive(Debug)]
pub enum ImageFormat {
    ImageFormatNone = -1,
    ImageFormatRgba8888 = 0,
    ImageFormatAbgr8888,
    ImageFormatRgb888,
    ImageFormatBgr888,
    ImageFormatRgb565,
    ImageFormatI8,
    ImageFormatIa88,
    ImageFormatP8,
    ImageFormatA8,
    ImageFormatRgb888Bluescreen,
    ImageFormatBgr888Bluescreen,
    ImageFormatArgb8888,
    ImageFormatBgra8888,
    ImageFormatDxt1,
    ImageFormatDxt3,
    ImageFormatDxt5,
    ImageFormatBgrx8888,
    ImageFormatBgr565,
    ImageFormatBgrx5551,
    ImageFormatBgra4444,
    ImageFormatDxt1Onebitalpha,
    ImageFormatBgra5551,
    ImageFormatUv88,
    ImageFormatUvwq8888,
    ImageFormatRgba16161616f,
    ImageFormatRgba16161616,
    ImageFormatUvlx8888
}

impl ImageFormat {
    pub fn from(num: u32) -> ImageFormat {
        match num {
            13 => ImageFormat::ImageFormatDxt1,
            _ => panic!("ImageFormat {} not supported", num)
        }
    }
}
/*
typedef struct tagVTFHEADER
{
	char		signature[4];		// File signature ("VTF\0"). (or as little-endian integer, 0x00465456)
	unsigned int	version[2];		// version[0].version[1] (currently 7.2).
	unsigned int	headerSize;		// Size of the header struct  (16 byte aligned; currently 80 bytes) + size of the resources dictionary (7.3+).
	unsigned short	width;			// Width of the largest mipmap in pixels. Must be a power of 2.
	unsigned short	height;			// Height of the largest mipmap in pixels. Must be a power of 2.
	unsigned int	flags;			// VTF flags.
	unsigned short	frames;			// Number of frames, if animated (1 for no animation).
	unsigned short	firstFrame;		// First frame in animation (0 based).
	unsigned char	padding0[4];		// reflectivity padding (16 byte alignment).
	float		reflectivity[3];	// reflectivity vector.
	unsigned char	padding1[4];		// reflectivity padding (8 byte packing).
	float		bumpmapScale;		// Bumpmap scale.
	unsigned int	highResImageFormat;	// High resolution image format.
	unsigned char	mipmapCount;		// Number of mipmaps.
	unsigned int	lowResImageFormat;	// Low resolution image format (always DXT1).
	unsigned char	lowResImageWidth;	// Low resolution image width.
	unsigned char	lowResImageHeight;	// Low resolution image height.

	// 7.2+
	unsigned short	depth;			// Depth of the largest mipmap in pixels.
						// Must be a power of 2. Can be 0 or 1 for a 2D texture (v7.2 only).

	// 7.3+
	unsigned char	padding2[3];		// depth padding (4 byte alignment).
	unsigned int	numResources;		// Number of resources this vtf has
} VTFHEADER;
*/
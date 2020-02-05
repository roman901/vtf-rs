use crate::header::VTFHeader;
use crate::image::ImageFormat;
use crate::Error;

pub fn get_offset(
    header: &VTFHeader,
    image_format: &ImageFormat,
    frame: u32,
    face: u32,
    slice: u32,
    mip_level: i32,
) -> Result<u32, Error> {
    let mut offset: u32 = 0;

    for i in (mip_level + 1..(header.mipmap_count) as i32).rev() {
        offset += get_mip_size(header, image_format, i as u32, header.depth)?;
    }

    offset *= header.frames as u32;

    let volume_bytes: u32 = get_mip_size(header, image_format, mip_level as u32, header.depth)?;
    let slice_bytes: u32 = get_mip_size(header, image_format, mip_level as u32, 1)?;

    offset += volume_bytes * (frame + face);
    offset += slice_bytes * slice;

    Ok(offset)
}

pub fn get_mip_size(
    header: &VTFHeader,
    image_format: &ImageFormat,
    mip_level: u32,
    depth: u16,
) -> Result<u32, Error> {
    let mut mip_width = header.width.wrapping_shr(mip_level);
    let mut mip_height = header.height.wrapping_shr(mip_level);
    let mut mip_depth = depth.wrapping_shr(mip_level);

    if mip_width < 1 {
        mip_width = 1;
    }

    if mip_height < 1 {
        mip_height = 1;
    }
    if mip_depth < 1 {
        mip_depth = 1;
    }

    Ok(image_format.frame_size(mip_width as u32, mip_height as u32)? * mip_depth as u32)
}

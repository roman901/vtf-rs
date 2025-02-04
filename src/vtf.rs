use crate::header::VTFHeader;
use crate::image::{ImageFormat, VTFImage};
use crate::resources::{ResourceList, ResourceType};
use crate::Error;
use image::{DynamicImage};
use std::io::Cursor;
use std::vec::Vec;
use texpresso::{Format, Params};

#[derive(Debug)]
pub struct VTF<'a> {
    pub header: VTFHeader,
    pub lowres_image: VTFImage<'a>,
    pub highres_image: VTFImage<'a>,
}

impl<'a> VTF<'a> {
    pub fn read(bytes: &'a Vec<u8>) -> Result<VTF, Error> {
        let mut cursor = Cursor::new(bytes);

        let header = VTFHeader::read(&mut cursor)?;

        let lowres_offset = match header
            .resources
            .get_by_type(ResourceType::VTF_LEGACY_RSRC_LOW_RES_IMAGE)
        {
            Some(resource) => resource.data,
            None => header.header_size,
        };

        let highres_offset = match header
            .resources
            .get_by_type(ResourceType::VTF_LEGACY_RSRC_IMAGE)
        {
            Some(resource) => resource.data,
            None => {
                lowres_offset
                    + header.lowres_image_format.frame_size(
                        header.lowres_image_width as u32,
                        header.lowres_image_height as u32,
                    )?
            }
        };

        let lowres_image = VTFImage::new(
            header.clone(),
            header.lowres_image_format,
            header.lowres_image_width as u16,
            header.lowres_image_height as u16,
            bytes,
            lowres_offset as usize,
        );

        let highres_image = VTFImage::new(
            header.clone(),
            header.highres_image_format,
            header.width,
            header.height,
            bytes,
            highres_offset as usize,
        );

        Ok(VTF {
            header,
            lowres_image,
            highres_image,
        })
    }

    pub fn create(image: DynamicImage, image_format: ImageFormat) -> Result<Vec<u8>, Error> {
        if !image.width().is_power_of_two()
            || !image.height().is_power_of_two()
            || image.width() > u16::max_value() as u32
            || image.height() > u16::max_value() as u32
        {
            return Err(Error::InvalidImageSize);
        }

        let header = VTFHeader {
            signature: VTFHeader::SIGNATURE,
            version: [7, 1], // simpler version without resources for now
            header_size: 64,
            width: image.width() as u16,
            height: image.height() as u16,
            flags: 8972,
            frames: 1,
            first_frame: 0,
            reflectivity: [0.0, 0.0, 0.0],
            bumpmap_scale: 1.0,
            highres_image_format: image_format,
            mipmap_count: 1,
            lowres_image_format: ImageFormat::Dxt1, // always the case
            lowres_image_width: 0,                  // no lowres for now
            lowres_image_height: 0,
            depth: 1,
            resources: ResourceList::empty(),
        };

        let mut data = Vec::with_capacity(
            header.header_size as usize
                + header
                    .highres_image_format
                    .frame_size(header.width as u32, header.height as u32)?
                    as usize
                + header.lowres_image_format.frame_size(
                    header.lowres_image_width as u32,
                    header.lowres_image_height as u32,
                )? as usize,
        );

        header.write(&mut data)?;

        let header_size = header.size();
        assert!(data.len() <= header_size, "invalid header size");

        data.resize(header_size, 0);

        let width = header.width as usize;
        let height = header.height as usize;

        match image_format {
            ImageFormat::Dxt5 => {
                let image_data = image.to_rgba8();
                data.resize(header_size + Format::Bc3.compressed_size(width, height), 0);
                Format::Bc3.compress(
                    image_data.as_raw(),
                    width,
                    height,
                    Params::default(),
                    &mut data[header_size..],
                );
            }
            ImageFormat::Dxt1Onebitalpha => {
                let image_data = image.to_rgba8();
                data.resize(header_size + Format::Bc1.compressed_size(width, height), 0);
                Format::Bc1.compress(
                    image_data.as_raw(),
                    width,
                    height,
                    Params::default(),
                    &mut data[header_size..],
                );
            }
            ImageFormat::Rgba8888 => {
                let image_data = image.to_rgba8();
                data.extend_from_slice(&image_data);
            }
            ImageFormat::Rgb888 => {
                let image_data = image.to_rgb8();
                data.extend_from_slice(&image_data);
            }
            _ => return Err(Error::UnsupportedEncodeImageFormat(image_format)),
        }

        Ok(data)
    }
}

extern crate image;
extern crate vtf;

use image::dxt::{DXTDecoder, DXTVariant};
use image::DecodingResult::U8;
use image::DynamicImage;
use image::ImageBuffer;
use image::ImageDecoder;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;

fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        panic!("Usage: png <path to vtf file> <destination of new png file");
    }

    let path = Path::new(&args[1]);
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let vtf = vtf::from_bytes(&mut buf)?;
    let bytes = vtf.highres_image.get_frame(0);

    let mut dxt_decoder = DXTDecoder::new(
        &bytes[..],
        vtf.highres_image.width as u32,
        vtf.highres_image.height as u32,
        DXTVariant::DXT1,
    )
    .unwrap();
    let buf = dxt_decoder.read_image().unwrap();

    let image = match buf {
        U8(buf) => Some(
            ImageBuffer::from_raw(
                vtf.highres_image.width as u32,
                vtf.highres_image.height as u32,
                buf,
            )
            .map(DynamicImage::ImageRgb8)
            .unwrap(),
        ),
        _ => None,
    };
    let img = image.unwrap();

    img.save(&args[2])?;
    Ok(())
}

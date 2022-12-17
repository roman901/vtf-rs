use image::DynamicImage;
use std::env;
use std::fs;
use vtf::Error;

fn main() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        panic!("Usage: png <path to vtf file> <destination of new png file>");
    }

    let mut buf = fs::read(&args[1])?;

    let vtf = vtf::from_bytes(&mut buf)?;
    let image = vtf.highres_image.decode(0)?;

    // rgb and rgba images we can save directly, for other formats we convert to rgba
    match image {
        DynamicImage::ImageRgb8(_) => image.save(&args[2])?,
        DynamicImage::ImageRgba8(_) => image.save(&args[2])?,
        DynamicImage::ImageBgra8(_) => image.to_rgba8().save(&args[2])?,
        _ => image.to_rgb8().save(&args[2])?,
    };
    Ok(())
}

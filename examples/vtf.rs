use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::vec::Vec;
use vtf::{Error, ImageFormat};

fn main() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        panic!("Usage: vtf <path to png file> <destination of new vtf file");
    }

    let image = image::open(&args[1])?;
    let vtf_data = vtf::create(image, ImageFormat::Dxt5)?;

    let path = Path::new(&args[2]);
    let mut file = File::create(path)?;
    file.write(&vtf_data)?;

    Ok(())
}

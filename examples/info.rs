use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;
use vtf::Error;

fn main() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: info <path to vtf file>");
    }

    let path = Path::new(&args[1]);
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let vtf = vtf::from_bytes(&buf)?;

    println!("{:#?}", vtf.header);
    Ok(())
}

use std::{fs::File, io::BufReader};
use byteorder::{ReadBytesExt};

pub fn run(file: &str) -> anyhow::Result<()> {
    println!("Hello World!");
    // open the file and get a reader out of it.
    let f = File::open(file)?;
    let mut reader = BufReader::new(f);

    let mut _byte1 = reader.read_u8()?;

    dbg!(_byte1);

    anyhow::Result::Ok(())
}
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;

use bstr::{io::BufReadExt, ByteSlice};
use csv;

mod lib;

fn main() -> std::io::Result<()> {
    let mut reader = BufReader::new(io::stdin());
    let mut writer = csv::Writer::from_writer(io::stdout());
    lib::parse_gdmi(&mut reader, &mut writer)?;
    Ok(())
}

use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;

use bstr::{ByteSlice, io::BufReadExt};
use csv;

mod lib;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let path = Path::new(filename);
    let display = path.display();

    let input_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(input_file) => input_file,
    };

    let mut reader = BufReader::new(input_file);
    let mut writer = csv::Writer::from_writer(io::stdout());

    lib::parse_gdmi(&mut reader, &mut writer)?;

    // reader.for_byte_line(|line| {
    //     writer.write_record(&[
    //         &line[0..9],
    //         &line[32..142].trim_end(),
    //     ])?;
    //     writer.flush()?;
    //     Ok(true)
    // })?;

    Ok(())
}

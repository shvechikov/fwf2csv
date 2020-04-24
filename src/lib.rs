use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use bstr::{ByteSlice, io::BufReadExt};
use csv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ConfigRecord {
    column: String,
    start: usize,
    length: usize,
}

fn get_config_file() -> File {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let path = Path::new(filename);
    let display = path.display();

    let input_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(input_file) => input_file,
    };
    input_file
}

fn read_config() -> Result<Vec<ConfigRecord>, Box<dyn Error>> {
    let config_file = get_config_file();
    let mut csv_reader = csv::Reader::from_reader(config_file);
    let mut config: Vec<ConfigRecord> = Vec::new();

    for result in csv_reader.deserialize() {
        let record: ConfigRecord = result?;
        config.push(record);
    }
    Ok(config)
}


fn parse_line<'a>(line: &'a [u8], config: &'a Vec<ConfigRecord>) -> impl Iterator<Item = &'a [u8]> {
    config
        .iter()
        .map(move |f| line[f.start..(f.start + f.length)].trim_end())
}

pub fn parse_gdmi() -> io::Result<()> {
    let reader = BufReader::new(io::stdin());
    let mut writer = csv::Writer::from_writer(io::stdout());
    let config = read_config().unwrap();

    reader.for_byte_line(|line| {
        let lazy_record = parse_line(&line, &config);
        writer.write_record(lazy_record)?;
        writer.flush()?;
        Ok(true)
    })
}

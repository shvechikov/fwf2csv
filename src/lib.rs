use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};

use bstr::{io::BufReadExt, ByteSlice};
use csv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ConfigRecord {
    column: String,
    start: usize,
    length: usize,
}

fn read_config(schema_file: File) -> Result<Vec<ConfigRecord>, Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_reader(schema_file);
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

pub fn parse_gdmi(schema_file: File) -> io::Result<()> {
    let reader = BufReader::new(io::stdin());
    let mut writer = csv::Writer::from_writer(io::stdout());
    let config = read_config(schema_file).unwrap();

    reader.for_byte_line(|line| {
        let lazy_record = parse_line(&line, &config);
        writer.write_record(lazy_record)?;
        writer.flush()?;
        Ok(true)
    })
}

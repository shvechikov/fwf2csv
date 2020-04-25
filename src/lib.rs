use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};

use bstr::{io::BufReadExt, ByteSlice};
use csv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SchemaRecord {
    column: String,
    start: usize,
    length: usize,
}

fn read_schema(schema_file: File) -> Result<Vec<SchemaRecord>, Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_reader(schema_file);
    let mut schema: Vec<SchemaRecord> = Vec::new();

    for result in csv_reader.deserialize() {
        let record: SchemaRecord = result?;
        schema.push(record);
    }
    Ok(schema)
}

fn parse_line<'a>(line: &'a [u8], schema: &'a Vec<SchemaRecord>) -> impl Iterator<Item = &'a [u8]> {
    schema
        .iter()
        .map(move |f| line[f.start..(f.start + f.length)].trim())
}

pub fn parse_gdmi(schema_file: File) -> io::Result<()> {
    let reader = BufReader::new(io::stdin());
    let mut writer = csv::Writer::from_writer(io::stdout());
    let schema = read_schema(schema_file).unwrap();

    reader.for_byte_line(|line| {
        let lazy_record = parse_line(&line, &schema);
        writer.write_record(lazy_record)?;
        writer.flush()?;
        Ok(true)
    })
}

use clap::{App, Arg};
use std::fs::File;
use std::path::Path;
use std::process;
mod lib;

const AFTER_HELP: &'static str = "\
EXAMPLE:
    cat input_file.fwf | fwf2csv input_schema.csv > output.csv

SCHEMA FILE EXAMPLE:
    > cat input_schema.csv
    column,start,length
    field_1,0,9
    field_2,9,5
    field_3,14,4
";

fn main() {
    let matches = App::new("fwf2csv")
        .version("0.1.0")
        .about("Converts fixed-width files (FWF) to comma separated (CSV).")
        .after_help(AFTER_HELP)
        .arg(
            Arg::with_name("schema")
                .required(true)
                .help("Fixed-width schema file")
                .takes_value(true),
        )
        .get_matches();

    let filename = matches.value_of("schema").unwrap();
    let path = Path::new(filename);
    let display = path.display();
    let schema_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(schema_file) => schema_file,
    };

    if let Err(err) = lib::parse_gdmi(schema_file) {
        println!("{}", err);
        process::exit(1);
    }
}

use std::process;

mod lib;

fn main() {
    if let Err(err) = lib::parse_gdmi() {
        println!("{}", err);
        process::exit(1);
    }
}

fwf2csv 0.1.0
Converts fixed-width files (FWF) to comma separated (CSV).

USAGE:
    fwf2csv <schema>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <schema>    Fixed-width schema file

EXAMPLE:
    cat input_file.fwf | fwf2csv input_schema.csv > output.csv

SCHEMA FILE EXAMPLE:
    > cat input_schema.csv
    column,start,length
    field_1,0,9
    field_2,9,5
    field_3,14,4

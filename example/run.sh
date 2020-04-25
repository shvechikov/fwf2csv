#! /usr/bin/env bash
cat input_file.fwf | ../target/debug/fwf2csv input_schema.csv

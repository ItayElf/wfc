use std::io::ErrorKind::InvalidInput;
use std::io::{Error, Write};
use std::{fs::File, io::Read};

use crate::wfc::parsing::json_parse::rules_to_json;
use crate::wfc::parsing::text_parse::generate_rules;

static DEFAULT_OUTPUT_FILE: &str = "rules.json";

/// Compiles a text file into rules json
pub fn compile(
    input_file_name: Option<&String>,
    output_file_name: Option<&String>,
) -> Result<(), Error> {
    let input_file_name =
        input_file_name.ok_or(Error::new(InvalidInput, "No input file was given"))?;

    let output_file_name = match output_file_name {
        Some(file) => file,
        None => DEFAULT_OUTPUT_FILE,
    };

    let mut input_file = File::open(input_file_name)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    let rules = generate_rules(contents);
    let json = rules_to_json(&rules)?;

    let mut output_file = File::create(output_file_name)?;
    output_file.write_all(json.as_bytes())?;

    Ok(())
}

use std::fs::File;
use std::io::ErrorKind::{InvalidData, InvalidInput};
use std::io::{Error, Read};

use crate::wfc::parsing::json_parse::parse_rules_json;
use crate::wfc::parsing::post_processing::merge;
use crate::wfc::wfc::algorithm::iterate;
use crate::wfc::wfc::rules::generate_wfc_vector;

/// Generates a text with [tokens] tokens and rules from the given file
pub fn generate(rules_file_name: Option<&String>, tokens: Option<&usize>) -> Result<(), Error> {
    let rules_file_name =
        rules_file_name.ok_or(Error::new(InvalidInput, "No rules file was given"))?;
    let tokens = tokens.ok_or(Error::new(InvalidInput, "Field tokens is empty"))?;

    let mut input_file = File::open(rules_file_name)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    let rules = parse_rules_json(&contents)?;
    let vector = generate_wfc_vector(&rules, *tokens);

    let result = iterate(vector, &rules);

    match result {
        Ok(result) => {
            print!("{}", merge(result));
            Ok(())
        }
        Err(e) => Err(Error::new(InvalidData, e)),
    }
}

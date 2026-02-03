extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::error::Error;
use pest::Parser;

use std::env;
use std::fmt;
use std::fs;

#[derive(Parser)]
#[grammar = "csharp6.0_grammar.pest"]
pub struct KestrosCSharpToDart;

#[derive(Debug)]
pub struct OsStrError<'a> {
    description: &'a str,
}

impl fmt::Display for OsStrError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OsStrError: {}", self.description)
    }
}

impl Error for OsStrError<'_> {
    fn description(&self) -> &'_ str {
        self.description
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let dir = env::current_dir()?;
    println!("{:?}", dir);
    let cur_dir_os_str = dir.as_os_str();
    let cur_dir_option: Option<&str> = cur_dir_os_str.to_str();
    let cur_dir = match cur_dir_option {
        Some(x) => {
            x
        },
        None => {
            return Result::Err(
                Box::new(
                    OsStrError{
                        description: "Conversion of osstr failed."
                    }
                )
            );
        }
    };
    let input_file_path_in_project = "/src/input.txt";
    let mut input_file_path: String = String::from(cur_dir);
    input_file_path.push_str(
        input_file_path_in_project
    );
    println!("{}", input_file_path);
    let guts = fs::read_to_string::<&str>(&input_file_path)?;
    let parsed_guts = guts.parse::<String>()?;
    let successful_parse =
        KestrosCSharpToDart::parse(Rule::compilation_unit, &parsed_guts)?;
    println!("{:?}", &successful_parse);
    for token in successful_parse.tokens() {
        println!("{:?}", token);
    }
    Ok(())
}

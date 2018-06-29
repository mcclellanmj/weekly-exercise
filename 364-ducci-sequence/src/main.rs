extern crate ducci;

use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::error::Error;
use std::fmt::{Formatter, Display};

#[derive(Debug)]
enum DucciError {
    ParseError(&'static str)
}

impl From<ParseIntError> for DucciError {
    fn from(_: ParseIntError) -> Self {
        DucciError::ParseError("Error while parsing integers")
    }
}

impl Error for DucciError {
    fn description(&self) -> &str {
        match *self {
            DucciError::ParseError(desc) => desc
        }
    }
}

impl Display for DucciError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            DucciError::ParseError(desc) => write!(f, "Error during parsing: {}", desc)
        }
    }
}

fn parse_input(input: String) -> Result<Vec<u32>, DucciError> {
    let mut mut_input = input;
    let first = mut_input.remove(0);
    if first != '(' {
        return Err(DucciError::ParseError("Input should begin with ("));
    }

    let last = mut_input.pop().unwrap();
    if last != ')' {
        return Err(DucciError::ParseError("Input should end with )"));
    }

    let no_spaces = mut_input.replace(' ',"");

    let return_val: Result<Vec<u32>, ParseIntError> = no_spaces.split(',')
    .map(|x| x.parse::<u32>()).collect();

    return_val.map_err(From::from)
}

fn format_line(row: &Vec<u32>) -> String {
    let as_strings: Vec<String> = row.iter().map(|x| x.to_string()).collect();
    let mut joined = as_strings.join("; ");

    joined.insert(0, '[');
    joined.push(']');

    joined
}

fn main() {
    let unlocked_stdin = io::stdin();
    let locked_stdin = unlocked_stdin.lock();

    locked_stdin.lines().for_each(|read_result| {
        let parsed_input = parse_input(read_result.unwrap());
        match parsed_input {
            Ok(x) => {
                let sequence: ducci::DucciIterator = ducci::DucciIterator::new(x);
                let mut count = 0;

                for piece in sequence {
                    println!("{}", format_line(&piece));
                    count = count + 1;
                }
                println!("{} steps", count)
            },
            Err(x) => {
                println!("{}", x);
            }
        }
    });
}

extern crate ducci;

use std::io;
use std::io::BufRead;
use std::num::ParseIntError;

#[derive(Debug)]
enum Error {
    ParseError(&'static str)
}

fn parse_input(input: String) -> Result<Vec<u32>, Error> {
    let mut mut_input = input;
    let first = mut_input.remove(0);
    if first != '(' {
        return Err(Error::ParseError("Input should begin with ("));
    }

    let last = mut_input.pop().unwrap();
    if last != ')' {
        return Err(Error::ParseError("Input should end with )"));
    }

    let no_spaces = mut_input.replace(' ',"");

    let return_val: Result<Vec<u32>, ParseIntError> = no_spaces.split(',')
        .map(|x| x.parse::<u32>()).collect();

    Ok(return_val.unwrap())
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
        let sequence: Vec<Vec<u32>> = ducci::calculate_sequence(parsed_input.unwrap());

        for piece in &sequence {
            println!("{}", format_line(piece));
        }
        println!("{} steps", sequence.len())
    });
}

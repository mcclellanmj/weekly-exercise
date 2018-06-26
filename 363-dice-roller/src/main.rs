extern crate rand;

use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::error::Error;
use std::fmt::Formatter;
use rand::{thread_rng, Rng};

#[derive(Debug)]
struct DiceType {
    number_of_dice: u8,
    sides_of_dice: u8
}

#[derive(Debug)]
enum DiceParseError {
    NumberParseError(ParseIntError),
    StringFormatError(usize),
    ConstraintError
}

impl Error for DiceParseError {
    fn description(&self) -> &str {
        match *self {
            DiceParseError::NumberParseError(_) => "Invalid number given",
            DiceParseError::StringFormatError(_) => "Invalid string given, required format is 1d1",
            DiceParseError::ConstraintError => "Invalid piece given, numbers can be maximum of 100"
        }
    }
}

impl std::fmt::Display for DiceParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            DiceParseError::NumberParseError(ref parse_err) => parse_err.fmt(f),
            DiceParseError::StringFormatError(length) => write!(f, "Expected to parse out 2 numbers but got {}", length),
            DiceParseError::ConstraintError => write!(f, "Expected max of 100 exceeded")
        }
    }
}

fn parse_dice_type(dice_string: String) -> Result<DiceType, DiceParseError> {
    let results: Result<Vec<u8>, ParseIntError> = dice_string
        .to_lowercase()
        .split('d')
        .map(|x: &str| {
            x.parse::<u8>()
        }).collect();

    match results {
        Ok(numbers) => {
            let size = numbers.len();
            if size != 2 {
                Err(DiceParseError::StringFormatError(size))
            } else if numbers.iter().any(|x| *x > 100) {
                Err(DiceParseError::ConstraintError)
            } else {
                Ok(DiceType {
                    number_of_dice: numbers[0],
                    sides_of_dice: numbers[1]
                })
            }
        },
        Err(e) => Err(DiceParseError::NumberParseError(e))
    }
}

fn roll_dice(dice: &DiceType) -> Vec<u8> {
    let mut rng = thread_rng();

    let range = 0..dice.number_of_dice;
    range.map(|_| {
        rng.gen_range(1, dice.sides_of_dice + 1)
    }).collect()
}

fn create_roll_output(rolls: &Vec<u8>) -> String {
    let as_strings: Vec<String> = rolls.iter().map(|x| x.to_string()).collect();
    as_strings.join(", ")
}

fn print_results(dice: Result<DiceType, DiceParseError>) {
    match dice {
        Ok(dice) => {
            let rolls = roll_dice(&dice);
            let total: u32 = rolls.iter().fold(0, |sum, i| sum + *i as u32);
            let roll_string = create_roll_output(&rolls);

            println!("{}: [{}]", total, roll_string);
        },
        Err(err) => println!("{}", err)
    }
}

fn main() {
    let unlocked_stdin = io::stdin();
    let locked_stdin = unlocked_stdin.lock();

    locked_stdin.lines().for_each(|read_result| {
        match read_result {
            Ok(val) => print_results(parse_dice_type(val)),
            Err(error) => println!("{:?}", error)
        }
    });
}

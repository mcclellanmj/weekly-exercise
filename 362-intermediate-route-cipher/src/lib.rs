use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TransRoute {
    Spiral,
    ReverseSpiral
}

#[derive(Debug)]
enum Direction {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize)
}

#[derive(Debug)]
pub struct TransMatrix<'a> {
    array: Vec<char>,
    dimensions: &'a (usize, usize)
}

#[derive(Debug)]
struct SpiralIterator {
    dimensions: (usize, usize),
    remaining_x: usize,
    remaining_y: usize,
    current_position: (usize, usize),
    end: bool,
    direction: Direction
}

impl SpiralIterator {
    fn get_next_direction(&mut self) -> Direction {
        match self.direction {
            Direction::Up(remaining) => {
                if remaining > 1 {
                    Direction::Up(remaining - 1)
                } else {
                    self.remaining_x = self.remaining_x - 1;
                    Direction::Right(self.remaining_x)
                }
            },
            Direction::Down(remaining) => {
                if remaining > 1 {
                    Direction::Down(remaining - 1)
                } else {
                    self.remaining_x = self.remaining_x - 1;
                    Direction::Left(self.remaining_x)
                }
            },
            Direction::Left(remaining) => {
                if remaining > 1 {
                    Direction::Left(remaining - 1)
                } else {
                    self.remaining_y = self.remaining_y - 1;
                    Direction::Up(self.remaining_y)
                }
            },
            Direction::Right(remaining) => {
                if remaining > 1 {
                    Direction::Right(remaining - 1)
                } else {
                    self.remaining_y = self.remaining_y - 1;
                    Direction::Down(self.remaining_y)
                }
            }
        }
    }

    fn get_next_position(current_direction: &Direction, current_position: &(usize, usize)) -> (usize, usize) {
        match current_direction {
            Direction::Up(_) => (current_position.0, current_position.1 - 1),
            Direction::Down(_) => (current_position.0, current_position.1 + 1),
            Direction::Left(_) => (current_position.0 - 1, current_position.1),
            Direction::Right(_) => (current_position.0 + 1, current_position.1),
        }
    }
}

impl Iterator for SpiralIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.end {
            None
        } else if self.remaining_x == 0 || self.remaining_y == 0 {
            self.end = true;
            Some(self.current_position)
        } else {
            let next_value = self.current_position;

            println!("position: [{:?}]", self.current_position);
            println!("direction: [{:?}]", self.direction);

            self.current_position = SpiralIterator::get_next_position(&self.direction, &self.current_position);
            self.direction = self.get_next_direction();

            println!("next_value was: [{:?}]", next_value);
            Some(next_value)
        }
    }
}

#[derive(Debug)]
pub struct TransCipher {
    dimensions: (usize, usize)
}

#[derive(Debug)]
pub struct OutOfBoundsError {
    x: usize,
    y: usize,
    dimensions: (usize, usize)
}

impl Error for OutOfBoundsError {
    fn description(&self) -> &str {
        "Tried to access out of bounds"
    }
}

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tried to access x={} and y={} but max x={} and max y={}"
            , self.x
            , self.y
            , self.dimensions.0 - 1
            , self.dimensions.1 - 1)
    }
}

impl TransCipher {
    pub fn new(x: usize, y: usize) -> TransCipher {
        TransCipher {
            dimensions: (x, y)
        }
    }

    pub fn build_matrix<'a, S: Into<String>>(&'a self, to_encode: S) -> TransMatrix<'a> {
        TransMatrix {
            array: to_encode.into().to_uppercase().chars().filter(|x| x.is_alphabetic()).collect(),
            dimensions: &self.dimensions
        }
    }

    pub fn encode<S: Into<String>>(&self, route: TransRoute, to_encode: S) -> String {
        let matrix = self.build_matrix(to_encode);

        let remaining_x = self.dimensions.0;
        let remaining_y = self.dimensions.1;

        let spiral_iterator = SpiralIterator {
            dimensions: self.dimensions,
            remaining_x,
            remaining_y,
            end: false,
            current_position: (self.dimensions.0 - 1, 0),
            direction: Direction::Down(remaining_y - 1)
        };

        return spiral_iterator.map(|x| matrix.get_char(x).unwrap()).collect();
    }
}

impl <'a> TransMatrix<'a> {
    pub fn get_char(&self, input: (usize, usize)) -> Result<&char, OutOfBoundsError> {
        let (x, y) = input;
        if x > self.dimensions.0 - 1 || y > self.dimensions.1 - 1 {
            Err(
                OutOfBoundsError {
                    x,
                    y,
                    dimensions: self.dimensions.clone()
                }
            )

        } else {
            let target_index = x + (y * self.dimensions.0);

            Ok(self.array.get(target_index).unwrap_or(&'X'))
        }
    }
}

#[cfg(test)]
mod tests {
    use TransCipher;
    use TransRoute;

    #[test]
    fn build_matrix() {
        let cipher = TransCipher::new(9, 3);
        let matrix = cipher.build_matrix("mATt");

        assert_eq!(&'M', matrix.get_char((0, 0)).unwrap());
        assert_eq!(&'A', matrix.get_char((1, 0)).unwrap());
        assert_eq!(&'T', matrix.get_char((2, 0)).unwrap());
        assert_eq!(&'T', matrix.get_char((3, 0)).unwrap());
        assert_eq!(&'X', matrix.get_char((3, 1)).unwrap());
        assert_eq!(&'X', matrix.get_char((8, 2)).unwrap());

        assert!(matrix.get_char((9, 0)).is_err(), "Expected failure due out of bounds");
        assert!(matrix.get_char((9, 4)).is_err(), "Expected failure due out of bounds");
    }
}

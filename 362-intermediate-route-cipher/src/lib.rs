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
struct SpiralIterator<'a> {
    matrix: &'a TransMatrix<'a>,
    remaining_x: usize,
    remaining_y: usize,
    current_position: (usize, usize),
    direction: Direction
}

impl <'a> Iterator for SpiralIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.remaining_x == 0 || self.remaining_y == 0 {
            // FIXME: Last value would never be returned
            None
        } else {
            let next_value =
                self.matrix.get_char(self.current_position.0, self.current_position.1).unwrap();

            let current_position = &mut self.current_position;

            match self.direction {
                Direction::Left(remaining) => {
                    *current_position = (current_position.0 - 1, current_position.1);

                    if remaining <= 1 {
                        self.direction = Direction::Up(self.remaining_y);
                        self.remaining_y = self.remaining_y - 1;
                    } else {
                        self.direction = Direction::Left(remaining - 1);
                    }
                },
                Direction::Right(remaining) => {
                    *current_position = (current_position.0 + 1, current_position.1);

                    if remaining <= 1 {
                        self.direction = Direction::Down(self.remaining_y);
                        self.remaining_y = self.remaining_y - 1;
                    } else {
                        self.direction = Direction::Right(remaining - 1);
                    }
                },
                Direction::Up(remaining) => {
                    *current_position = (current_position.0, current_position.1 - 1);

                    if remaining <= 1 {
                        self.direction = Direction::Right(self.remaining_x);
                        self.remaining_x = self.remaining_x - 1;
                    } else {
                        self.direction = Direction::Up(remaining - 1);
                    }
                },
                Direction::Down(remaining) => {
                    *current_position = (current_position.0, current_position.1 + 1);

                    if remaining <= 1 {
                        self.direction = Direction::Left(self.remaining_x);
                        self.remaining_x = self.remaining_x - 1;
                    } else {
                        self.direction = Direction::Down(remaining - 1);
                    }
                }
            }

            Some(*next_value)
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
            array: to_encode.into().to_uppercase().chars().collect(),
            dimensions: &self.dimensions
        }
    }

    pub fn encode<S: Into<String>>(&self, route: TransRoute, to_encode: S) -> String {
        let matrix = self.build_matrix(to_encode);

        let remaining_x = &self.dimensions.0;
        let remaining_y = &self.dimensions.1;

        unimplemented!("Still working on this")
    }
}

impl <'a> TransMatrix<'a> {
    pub fn get_char(&self, x: usize, y:usize) -> Result<&char, OutOfBoundsError> {
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

        assert_eq!(&'M', matrix.get_char(0, 0).unwrap());
        assert_eq!(&'A', matrix.get_char(1, 0).unwrap());
        assert_eq!(&'T', matrix.get_char(2, 0).unwrap());
        assert_eq!(&'T', matrix.get_char(3, 0).unwrap());
        assert_eq!(&'X', matrix.get_char(3, 1).unwap());
        assert_eq!(&'X', matrix.get_char(8, 2).unwrap());

        assert!(matrix.get_char(9, 0).is_err(), "Expected failure due out of bounds");
        assert!(matrix.get_char(9, 4).is_err(), "Expected failure due out of bounds");
    }
}

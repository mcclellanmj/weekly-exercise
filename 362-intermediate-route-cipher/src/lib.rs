use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TransRoute {
    Spiral,
    ReverseSpiral
}

#[derive(Debug)]
enum Direction {
    Left, Right, Up, Down
}

#[derive(Debug)]
enum IterationStatus {
    Unstarted((usize, usize)), Started((usize, usize)), Finished
}

#[derive(Debug)]
pub struct TransMatrix<'a> {
    array: Vec<char>,
    dimensions: &'a (usize, usize)
}

#[derive(Debug)]
pub struct Movement {
    direction: Direction,
    remaining: usize
}

#[derive(Debug)]
struct SpiralIterator {
    bounds: (usize, usize),
    current_position: IterationStatus,
    movement: Movement
}

impl SpiralIterator {
    fn do_movement(&mut self) {
        if self.movement.remaining > 1 {
            self.movement.remaining = self.movement.remaining - 1;
        } else {
            match self.movement.direction {
                Direction::Up => {
                    self.bounds.0 = self.bounds.0 - 1;
                    self.movement.remaining = self.bounds.0;
                    self.movement.direction = Direction::Right;
                },
                Direction::Down => {
                    self.bounds.0 = self.bounds.0 - 1;
                    self.movement.remaining = self.bounds.0;
                    self.movement.direction = Direction::Left;
                },
                Direction::Left => {
                    self.bounds.1 = self.bounds.1 - 1;
                    self.movement.remaining = self.bounds.1;
                    self.movement.direction = Direction::Up;
                },
                Direction::Right => {
                    self.bounds.1 = self.bounds.1 - 1;
                    self.movement.remaining = self.bounds.1;
                    self.movement.direction = Direction::Down;
                }
            }
        }
    }

    fn get_next_position(current_direction: &Direction, current_position: &(usize, usize)) -> (usize, usize) {
        match current_direction {
            &Direction::Up => (current_position.0, current_position.1 - 1),
            &Direction::Down => (current_position.0, current_position.1 + 1),
            &Direction::Left => (current_position.0 - 1, current_position.1),
            &Direction::Right => (current_position.0 + 1, current_position.1),
        }
    }
}

impl Iterator for SpiralIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.current_position {
            IterationStatus::Unstarted(position) => {
                self.current_position = IterationStatus::Started(position);
                Some(position)
            },
            IterationStatus::Started(position) => {
                if self.bounds.0 == 0 || self.bounds.1 == 0 {
                    self.current_position = IterationStatus::Finished;
                } else {
                    self.current_position =
                        IterationStatus::Started(SpiralIterator::get_next_position(&self.movement.direction, &position));
                    self.do_movement();
                }
                Some(position)
            },
            IterationStatus::Finished => None
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

        let spiral_iterator = SpiralIterator {
            bounds: self.dimensions,
            current_position: IterationStatus::Unstarted((self.dimensions.0 - 1, 0)),
            movement: Movement {
                direction: Direction::Down,
                remaining: self.dimensions.1 - 1
            }
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

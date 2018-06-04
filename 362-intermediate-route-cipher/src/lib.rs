use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TransRoute {
    Spiral,
    ReverseSpiral
}

#[derive(Debug)]
pub struct TransCipher {
    dimensions: (usize, usize),
    route: TransRoute
}

#[derive(Debug)]
pub struct TransMatrix<'a> {
    array: Vec<char>,
    dimensions: &'a (usize, usize)
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
    pub fn new(x: usize, y: usize, route: TransRoute) -> TransCipher {
        TransCipher {
            dimensions: (x, y),
            route
        }
    }

    pub fn build_matrix<'a, S: Into<String>>(&'a self, to_encode: S) -> TransMatrix<'a> {
        TransMatrix {
            array: to_encode.into().to_uppercase().chars().collect(),
            dimensions: &self.dimensions
        }
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
        let cipher = TransCipher::new(9, 3, TransRoute::Spiral);
        let matrix = cipher.build_matrix("mATt");

        assert_eq!(&'M', matrix.get_char(0, 0).unwrap());
        assert_eq!(&'A', matrix.get_char(1, 0).unwrap());
        assert_eq!(&'T', matrix.get_char(2, 0).unwrap());
        assert_eq!(&'T', matrix.get_char(3, 0).unwrap());
        assert_eq!(&'X', matrix.get_char(3, 1).unwrap());
        assert_eq!(&'X', matrix.get_char(8, 2).unwrap());

        assert!(matrix.get_char(9, 0).is_err(), "Expected failure due out of bounds");
        assert!(matrix.get_char(9, 4).is_err(), "Expected failure due out of bounds");
    }
}

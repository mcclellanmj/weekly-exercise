use std::collections::HashSet;

pub struct DucciIterator {
    current: Vec<u32>,
    seen: HashSet<Vec<u32>>
}

impl DucciIterator {
    pub fn new(starting: Vec<u32>) -> DucciIterator {
        DucciIterator {
            current: starting,
            seen: HashSet::new()
        }
    }

    fn is_last(&self) -> bool {
        self.current.iter().all(|x| *x == 0u32) || self.seen.contains(&self.current)
    }

    fn calculate_next(&self) -> Vec<u32> {
        let sequence = &self.current;
        let mut copy = sequence.clone();

        let first = copy.remove(0);
        copy.push(first);

        sequence.iter().zip(copy)
            .map(|(x, y)| (*x as i32 - y as i32).abs() as u32)
            .collect()
    }
}

impl Iterator for DucciIterator {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Vec<u32>> {
        if self.is_last() {
            None
        } else {
            let next = self.calculate_next();
            self.seen.insert(self.current.clone());
            self.current = next;

            Some(self.current.clone())
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use calculate_next;

    #[test]
    fn test_next() {
        let test: Vec<u32> = vec!(0, 653, 1854, 4063);

        let result = calculate_next(&test);

        assert_eq!(vec!(653, 1201, 2209, 4063), result);
    }
}
*/
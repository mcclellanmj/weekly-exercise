fn calculate_next(sequence: &Vec<u32>) -> Vec<u32> {
    let mut copy = sequence.clone();

    let first = copy.remove(0);
    copy.push(first);

    sequence.iter().zip(copy)
        .map(|(x, y)| (*x as i32 - y as i32).abs() as u32)
        .collect()
}

pub fn calculate_sequence(starting: Vec<u32>) -> Vec<Vec<u32>> {
    let mut final_seq = vec!();

    let mut next = starting;

    loop {
        if final_seq.contains(&next) || next.iter().all(|x| *x == 0u32) {
            final_seq.push(next);
            break;
        } else {
            let tmp_next = calculate_next(&next);
            final_seq.push(next);
            next = tmp_next;
        }
    }

    final_seq
}

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
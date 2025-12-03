use std::fs;

pub fn exec_2_part_1(filename: String) -> i64 {
    let contents = fs::read_to_string(filename).expect("error");
    let split: Vec<&str> = contents.split("\n").collect();
    let first_row = split.first().unwrap();

    let ranges: Vec<&str> = first_row.split(",").collect();

    println!("{:?}", ranges);

    let mut parsed_range: Vec<&str>;
    let mut first: i64;
    let mut second: i64;
    let mut sum: i64 = 0;

    for range in ranges.iter() {
        parsed_range = range.split("-").collect();

        first = parsed_range.get(0).unwrap().parse().unwrap();
        second = parsed_range.get(1).unwrap().parse().unwrap();

        for n in first..=second {
            if halves_are_equal(n) {
                // println!("Counting as same halves: {}", n);
                sum += n as i64;
            }
        }
    }

    return sum;
}

pub fn exec_2_part_2(filename: String) -> i64 {
    let contents = fs::read_to_string(filename).expect("error");
    let split: Vec<&str> = contents.split("\n").collect();
    let first_row = split.first().unwrap();

    let ranges: Vec<&str> = first_row.split(",").collect();

    println!("{:?}", ranges);

    let mut parsed_range: Vec<&str>;
    let mut first: i64;
    let mut second: i64;
    let mut sum: i64 = 0;

    for range in ranges.iter() {
        // println!("{:?}", range);

        parsed_range = range.split("-").collect();

        first = parsed_range.get(0).unwrap().parse().unwrap();
        second = parsed_range.get(1).unwrap().parse().unwrap();

        for n in first..=second {
            if has_repeating_pattern(n) {
                println!("Counting as repeated patterns: {}", n);
                sum += n as i64;
            }
        }
    }

    return sum;
}

fn has_repeating_pattern(n: i64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Try patterns of length 1 up to half the string length
    for pattern_len in 1..=(len / 2) {
        if len % pattern_len != 0 {
            continue;
        }

        // Take the first `pattern_len` characters as the candidate pattern
        let pattern = &s[..pattern_len];

        // Flag to check if this pattern repeats along the string
        let mut repeated = true;

        // Check every consecutive slice of length `pattern_len`
        for i in (0..len).step_by(pattern_len) {
            // Compute the end index (make sure we don't go past the string)
            let end = (i + pattern_len).min(len);

            // Compare the current slice with the pattern
            if &s[i..end] != &pattern[..end - i] {
                repeated = false;
                break;
            }
        }

        // If the pattern repeated throughout the string, return true
        if repeated {
            return true;
        }
    }

    // No repeating pattern found
    false
}

fn halves_are_equal(n: i64) -> bool {
    let s = n.abs().to_string(); // handle negative numbers
    let len = s.len();

    // must have an even number of digits
    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    let (first, second) = s.split_at(half);

    first == second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            exec_2_part_1("input/day2/example.txt".to_string()),
            1227775554
        );
    }

    #[test]
    fn test_part_1_input() {
        assert_eq!(
            exec_2_part_1("input/day2/input.txt".to_string()),
            13108371860
        );
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(
            exec_2_part_2("input/day2/example.txt".to_string()),
            4174379265
        );
    }

    #[test]
    fn test_part_2_input() {
        assert_eq!(
            exec_2_part_2("input/day2/input.txt".to_string()),
            22471660255
        );
    }
}

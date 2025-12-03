use std::fs;

pub fn part_1(filename: String) -> u32 {
    let contents = fs::read_to_string(filename).expect("error");
    let split = contents.split("\n");
    let vec: Vec<&str> = split.filter(|s| s != &"").collect();

    let sum: u32 = vec.iter().map(|s| max_combo_only(s) as u32).sum();

    sum
}

pub fn part_2(filename: String) -> u64 {
    let contents = fs::read_to_string(filename).expect("error");
    let split = contents.split("\n");
    let vec: Vec<&str> = split.filter(|s| s != &"").collect();

    return vec.iter().map(|s| select_up_to_12_digits_window(s)).sum();
}

pub fn max_combo_only(s: &str) -> u8 {
    // We assume s is non-empty as requested, so unwrap the first char directly.
    let mut chars = s.chars();
    let first_ch = chars.next().unwrap(); // safe because caller promised non-empty
    let mut prev_max: u8 = first_ch.to_digit(10).unwrap() as u8;

    let mut max_combo: u8 = 0;

    for ch in chars {
        let digit = ch.to_digit(10).unwrap() as u8;

        // compute combo using prev_max (from previous iterations)
        let combo = prev_max
            .checked_mul(10)
            .and_then(|v| v.checked_add(digit))
            .unwrap_or(0);

        if combo > max_combo {
            max_combo = combo;
        }

        // update prev_max after computing combo
        if digit > prev_max {
            prev_max = digit;
        }
    }

    max_combo
}

const MAX_DIGITS: usize = 12;

pub fn select_up_to_12_digits_window(s: &str) -> u64 {
    let digits: Vec<u8> = s.bytes().map(|b| b - b'0').collect();

    let len = digits.len();
    if len <= MAX_DIGITS {
        // If there are <= 12 digits, return them all
        build_u64_from_digits(&digits);
    }

    let mut result: Vec<u8> = Vec::with_capacity(MAX_DIGITS);
    let mut start_idx: usize = 0;

    // For each pick i (0..MAX_DIGITS-1) choose the leftmost maximum digit in the window
    for pick_index in 0..MAX_DIGITS {
        let max_allowed_idx = len - (MAX_DIGITS - pick_index);

        // Find max digit and its leftmost position in [start_idx ..= max_allowed_idx]
        let mut best_digit = digits[start_idx];
        let mut best_pos = start_idx;
        for pos in start_idx..=max_allowed_idx {
            let val = digits[pos];
            if val > best_digit {
                best_digit = val;
                best_pos = pos;
                // early exit: can't beat '9'
                if best_digit == 9 {
                    break;
                }
            }
        }

        result.push(best_digit);
        start_idx = best_pos + 1;
    }

    build_u64_from_digits(&result)
}

/// Build a u64 value by concatenating all digits in `digits` in order.
/// Example: &[1,2,3] -> 123u64
fn build_u64_from_digits(digits: &[u8]) -> u64 {
    let mut value: u64 = 0;
    for &d in digits {
        value = value * 10 + d as u64;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("input/day3/example.txt".to_string()), 357);
    }

    #[test]
    fn test_part_1_input() {
        assert_eq!(part_1("input/day3/input.txt".to_string()), 17316);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("input/day3/example.txt".to_string()), 3121910778619);
    }

    #[test]
    fn test_part_2_input() {
        assert_eq!(part_2("input/day3/input.txt".to_string()), 171741365473332);
    }
}

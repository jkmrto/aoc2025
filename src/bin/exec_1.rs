use std::fs;

pub fn part_1(filename: String) -> i32 {
    let contents = fs::read_to_string(filename).expect("error");
    let split = contents.split("\n");
    let vec: Vec<&str> = split.collect();

    // Initialize empty vectors
    let mut direction: &str;
    let mut amount_str: &str;

    let mut pos: i32 = 50;
    let mut counter: i32 = 0;

    for item in vec.iter() {
        if item == &"" {
            break;
        }
        (direction, amount_str) = item.split_at(1);
        let amount: i32 = amount_str.parse().expect("Not a valid number");

        if direction == "L" {
            pos = pos - amount;
        } else if direction == "R" {
            pos = pos + amount;
        }

        pos = pos % 100; // remainder after division by 100

        if pos < 0 {
            pos = pos + 100
        }

        if pos == 0 {
            counter += 1;
        }
    }

    counter
}

pub fn part_2(filename: String) -> i32 {
    let contents = fs::read_to_string(filename).expect("error");
    let split = contents.split("\n");
    let vec: Vec<&str> = split.collect();

    // Initialize empty vectors
    let mut direction: &str;
    let mut amount_str: &str;

    let mut pos: i32 = 50;
    let mut init_pos: i32;
    let mut counter: i32 = 0;
    let mut whole_rotations: i32;

    for item in vec.iter() {
        if item == &"" {
            break;
        }

        (direction, amount_str) = item.split_at(1);
        let amount: i32 = amount_str.parse().expect("Not a valid number");

        init_pos = pos;

        if direction == "L" {
            pos = pos - amount;
        } else if direction == "R" {
            pos = pos + amount;
        }

        if pos == 0 {
            counter += 1;
            continue;
        }

        whole_rotations = pos / 100; // remainder after division by 100
        pos = pos % 100; // remainder after division by 100

        counter = whole_rotations.abs() + counter;

        if direction == "L" && pos == 0 {
            counter += 1;
            continue;
        }

        if pos < 0 {
            pos = pos + 100;
            if init_pos != 0 {
                counter += 1;
            }
            continue;
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("input/day1/example.txt".to_string()), 3);
    }

    #[test]
    fn test_part_1_input() {
        assert_eq!(part_1("input/day1/input.txt".to_string()), 1059);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("input/day1/example.txt".to_string()), 6);
    }

    #[test]
    fn test_part_2_input() {
        assert_eq!(part_2("input/day1/input.txt".to_string()), 6305);
    }
}

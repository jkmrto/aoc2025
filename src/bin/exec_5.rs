use std::fs;

pub fn load_data(filename: String) -> (Vec<(u64, u64)>, Vec<u64>) {
    let contents = fs::read_to_string(filename).expect("error");
    let split: Vec<&str> = contents.split("\n\n").collect();

    let ranges_raw: Vec<&str> = split.first().unwrap().split("\n").collect();
    let mut ranges: Vec<(u64, u64)> = ranges_raw
        .iter()
        .map(|s| {
            let parts: Vec<&str> = s.split('-').collect();
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            (start, end)
        })
        .collect();

    let selected: Vec<u64> = split[1]
        .split("\n")
        .filter(|s| s != &"")
        .map(|s| s.parse().unwrap())
        .collect();

    ranges.sort_by_key(|k| k.0);

    let mut flag_range_change: bool;

    loop {
        flag_range_change = false;
        for i in 0..ranges.len() - 1 {
            if ranges[i].1 >= ranges[i + 1].0 && ranges[i].1 <= ranges[i + 1].1 {
                ranges[i].0 = ranges[i].0;
                ranges[i].1 = ranges[i + 1].1;
                ranges.remove(i + 1);
                flag_range_change = true;
                break;
            }

            if ranges[i].0 <= ranges[i + 1].0 && ranges[i].1 >= ranges[i + 1].1 {
                ranges[i].0 = ranges[i].0;
                ranges[i].1 = ranges[i].1;
                ranges.remove(i + 1);
                flag_range_change = true;
                break;
            }

            if ranges[i].1 == ranges[i + 1].0 {
                ranges[i].0 = ranges[i].0;
                ranges[i].1 = ranges[i + 1].1;
                ranges.remove(i + 1);
                flag_range_change = true;
                break;
            }

            if ranges[i].0 == ranges[i + 1].0 && ranges[0].1 == ranges[i + 1].1 {
                ranges[i].0 = ranges[i].0;
                ranges[i].1 = ranges[i + 1].1;
                ranges.remove(i + 1);
                flag_range_change = true;
                break;
            }
        }

        if !flag_range_change {
            break;
        }
    }

    (ranges, selected)
}

pub fn part_2(filename: String) -> u64 {
    let (ranges, _selected) = load_data(filename);
    let mut count: u64 = 0;

    // Count all numbers between ranges
    // Doing a for loop over ranges
    for i in 0..ranges.len() {
        count += 1 + (ranges[i].1 - ranges[i].0) as u64;
    }

    count
}

pub fn part_1(filename: String) -> u32 {
    let (mut ranges, mut selected) = load_data(filename);

    for i in 0..ranges.len() - 1 {
        assert!(ranges[i].1 < ranges[i + 1].0);
        assert!(ranges[i].0 != ranges[i + 1].0);
    }

    selected.sort_by_key(|k| *k);

    let mut selected_counter: u64 = 0;

    for sel in &selected {
        for i in 0..ranges.len() {
            if *sel >= ranges[i].0 && *sel <= ranges[i].1 {
                selected_counter += 1;
                break;
            }
        }
    }

    selected_counter as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("input/day5/example.txt".to_string()), 3);
    }

    #[test]
    fn test_part_1_input() {
        assert_eq!(part_1("input/day5/input.txt".to_string()), 679);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("input/day5/example.txt".to_string()), 14);
    }

    #[test]
    fn test_part_2_input() {
        assert_eq!(part_2("input/day5/input.txt".to_string()), 358155203664116);
    }
}

use regex::Regex;
use std::fs;

pub fn part_2(filename: String) -> u64 {
    let contents = fs::read_to_string(filename).expect("error");
    let mut lines: Vec<&str> = contents.split("\n").filter(|s| s != &"").collect();

    let operations_raw: &str = lines.pop().unwrap();

    let re = Regex::new(r"([+*])\s*").unwrap();

    let mut operations: Vec<&str> = vec![];
    for op in re.find_iter(operations_raw) {
        operations.push(op.as_str())
    }

    let mut matrix = vec![vec![""; lines.len()]; operations.len()];
    let mut operation_numbers: &str;

    for i in 0..lines.len() {
        let mut line_copy = lines[i];

        for j in 0..operations.len() {
            let n_chars: usize;
            if j == operations.len() - 1 {
                n_chars = operations[j].len();
            } else {
                n_chars = operations[j].len() - 1;
            }

            (operation_numbers, line_copy) = line_copy.split_at(n_chars);
            matrix[j][i] = operation_numbers;

            if line_copy.len() == 0 {
                break;
            }
            (_, line_copy) = line_copy.split_at(1);
        }
    }

    let mut number_rebuilt: String;
    let mut number_rebuilt_vec: Vec<u32>;
    let mut matrix_numbers_rebuilt = vec![];

    for i in 0..matrix.len() {
        let n_chars: usize = matrix[i][0].len();

        number_rebuilt_vec = vec![];

        for char_index in 0..n_chars {
            number_rebuilt = String::new();
            for element_index in 0..matrix[i].len() {
                let char_to_add = &matrix[i][element_index][char_index..char_index + 1];
                if char_to_add != " " {
                    number_rebuilt.push_str(char_to_add);
                }
            }

            number_rebuilt_vec.push(number_rebuilt.parse::<u32>().unwrap());
        }

        matrix_numbers_rebuilt.push(number_rebuilt_vec.clone());
    }

    let mut sum: u64 = 0;
    let mut temp_sum: u64;

    for i in 0..matrix_numbers_rebuilt.len() {
        temp_sum = 0;
        for number in matrix_numbers_rebuilt[i].clone() {
            if temp_sum == 0 {
                temp_sum = number as u64;
                continue;
            }

            if operations[i as usize].chars().next().unwrap() == '*' {
                temp_sum = temp_sum * number as u64;
            }

            if operations[i as usize].chars().next().unwrap() == '+' {
                temp_sum = temp_sum + number as u64;
            }
        }

        sum += temp_sum;
    }

    sum
}

pub fn part_1(filename: String) -> u64 {
    let contents = fs::read_to_string(filename).expect("error");
    let mut lines: Vec<&str> = contents.split("\n").filter(|s| s != &"").collect();
    let mut mattrix: Vec<Vec<u64>> = Vec::new();

    let operations: Vec<&str> = lines
        .pop()
        .unwrap()
        .split(" ")
        .filter(|s| s != &"")
        .collect();

    for line in lines {
        mattrix.push(
            line.split(" ")
                .filter(|s| s != &"")
                .map(|s| s.parse().unwrap())
                .collect(),
        );
    }

    let total_lines = mattrix.len() as u64;
    let total_columns = mattrix[0].len() as u64;

    let mut results = vec![0; total_columns as usize];

    for i in 0..total_lines {
        for j in 0..total_columns {
            if i == 0 {
                results[j as usize] = mattrix[i as usize][j as usize];
                continue;
            }

            if operations[j as usize] == "*" {
                results[j as usize] = results[j as usize] * mattrix[i as usize][j as usize];
            }

            if operations[j as usize] == "+" {
                results[j as usize] = results[j as usize] + mattrix[i as usize][j as usize];
            }
        }
    }

    results.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("input/day6/example.txt".to_string()), 4277556);
    }

    #[test]
    fn test_part_1_input() {
        assert_eq!(part_1("input/day6/input.txt".to_string()), 5361735137219);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("input/day6/example.txt".to_string()), 3263827);
    }

    #[test]
    fn test_part_2_input() {
        assert_eq!(part_2("input/day6/input.txt".to_string()), 11744693538946);
    }
}

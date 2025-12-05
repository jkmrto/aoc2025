use std::fs;

pub fn part_1(filename: String) -> u32 {
    let grid = load_grid_from_file(filename);
    let (_updated_grid, removal_counter) = apply_removal_step(&grid);

    removal_counter
}

pub fn part_2(filename: String) -> u32 {
    let mut grid = load_grid_from_file(filename);

    let mut removal_counter: u32;
    let mut sum_counter: u32 = 0;

    loop {
        (grid, removal_counter) = apply_removal_step(&grid);
        sum_counter += removal_counter;

        if removal_counter == 0 {
            break;
        }
    }

    sum_counter
}

fn load_grid_from_file(filename: String) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(filename).expect("error");
    let split = contents.split("\n");
    let vec: Vec<&str> = split.filter(|s| s != &"").collect();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in vec {
        grid.push(line.chars().collect());
    }

    grid
}

fn apply_removal_step(grid: &Vec<Vec<char>>) -> (Vec<Vec<char>>, u32) {
    let mut forklift_counter: u32 = 0;
    let mut updated_grid = grid.clone();

    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            if grid[row][column] == '@' {
                let mut local_count: u8 = 0;

                // check up
                if row > 0 && grid[row - 1][column] == '@' {
                    local_count += 1;
                }

                // checks up-left
                if row > 0 && column > 0 && grid[row - 1][column - 1] == '@' {
                    local_count += 1;
                }

                // check up-right
                if row > 0 && column < grid[row].len() - 1 && grid[row - 1][column + 1] == '@' {
                    local_count += 1;
                }

                // check down
                if row < (grid.len() - 1) && grid[row + 1][column] == '@' {
                    local_count += 1;
                }

                // check down-left
                if row < (grid.len() - 1) && column > 0 && grid[row + 1][column - 1] == '@' {
                    local_count += 1;
                }

                // check down-right
                if row < (grid.len() - 1)
                    && column < grid[row].len() - 1
                    && grid[row + 1][column + 1] == '@'
                {
                    local_count += 1;
                }

                // check left
                if column > 0 && grid[row][column - 1] == '@' {
                    local_count += 1;
                }
                // check right
                if column < grid[column].len() - 1 && grid[row][column + 1] == '@' {
                    local_count += 1;
                }

                if local_count < 4 {
                    forklift_counter += 1;
                    updated_grid[row][column] = '.';
                }
            }
        }
    }

    (updated_grid, forklift_counter)
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("input/day4/example.txt".to_string()), 13);
    }

    #[test]
    fn test_part_1_input() {
        assert_eq!(part_1("input/day4/input.txt".to_string()), 1602);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("input/day4/example.txt".to_string()), 43);
    }

    #[test]
    fn test_part_2_input() {
        assert_eq!(part_2("input/day4/input.txt".to_string()), 9518);
    }
}

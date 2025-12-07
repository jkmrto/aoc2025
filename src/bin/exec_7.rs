use std::collections::HashMap;
use std::fs;

pub fn part_1(filename: String) -> u64 {
    let mut grid = load_grid_from_file(filename);

    let starting_beam_pos = grid[0].iter().position(|&c| c == 'S').unwrap();
    grid[1][starting_beam_pos] = '|';

    let mut n_splits = 0;

    for i in 1..grid.len() - 1 {
        for j in 0..grid[i].len() {
            if grid[i][j] == '|' && grid[i + 1][j] == '^' {
                grid[i + 1][j + 1] = '|';
                grid[i + 1][j - 1] = '|';
                n_splits += 1;
            }

            if grid[i][j] == '|' && grid[i + 1][j] != '^' {
                grid[i + 1][j] = '|';
            }
        }
    }

    n_splits
}

pub fn part_2(filename: String) -> u64 {
    let mut grid = load_grid_from_file(filename);

    let mut map: HashMap<(usize, usize), u64> = HashMap::new();

    let starting_beam_pos = grid[0].iter().position(|&c| c == 'S').unwrap();
    grid[1][starting_beam_pos] = '|';

    *map.entry((1, starting_beam_pos)).or_insert(0) += 1;

    for i in 1..grid.len() - 1 {
        for j in 0..grid[i].len() {
            if grid[i][j] == '|' && grid[i + 1][j] == '^' {
                grid[i + 1][j + 1] = '|';
                grid[i + 1][j - 1] = '|';

                let particles = map.get(&(i, j)).unwrap().clone();

                *map.entry((i + 1, j + 1)).or_insert(0) += particles;
                *map.entry((i + 1, j - 1)).or_insert(0) += particles;
            }

            if grid[i][j] == '|' && grid[i + 1][j] != '^' {
                grid[i + 1][j] = '|';

                let particles = map.get(&(i, j)).unwrap().clone();
                *map.entry((i + 1, j)).or_insert(0) += particles;
            }
        }
    }

    let mut total_particles: u64 = 0;

    for j in 0..grid[grid.len() - 1].len() {
        total_particles += map.get(&(grid.len() - 1, j)).unwrap_or(&0);
    }

    total_particles as u64
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("input/day7/example.txt".to_string()), 21);
    }

    #[test]
    fn test_part_1_input() {
        assert_eq!(part_1("input/day7/input.txt".to_string()), 1646);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("input/day7/example.txt".to_string()), 40);
    }

    #[test]
    fn test_part_2_input() {
        assert_eq!(part_2("input/day7/input.txt".to_string()), 32451134474991);
    }
}

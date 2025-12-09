use std::fs::read_to_string;

const MANIFOLD: char = 'S';
const SPLITTER: char = '^';

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut grid: Vec<Vec<char>> = read_file("src/day7/input.txt")
            .split('\n')
            .map(|line| line.chars().collect())
            .collect();

        let start_idx = grid[0]
            .iter()
            .position(|&c| MANIFOLD.eq(&c))
            .expect("Manifold needs to be in first row.");

        let mut result = 0;
        let mut mask: Vec<bool> = vec![false; grid.iter().len()];
        mask[start_idx] = true;
        for line in grid {
            let current = mask.clone();
            for (idx, char) in line.iter().enumerate() {
                if SPLITTER.eq(&char) {
                    if current[idx] == true {
                        mask[idx - 1] = true;
                        mask[idx] = false;
                        mask[idx + 1] = true;
                        result += 1;
                    }
                }
            }
        }

        assert_eq!(result, 1);
    }
}

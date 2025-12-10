use std::fs::read_to_string;

const MANIFOLD: char = 'S';
const SPLITTER: char = '^';
const SPACE: char = '*';

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let grid: Vec<Vec<char>> = read_file("src/day7/input.txt").lines()
            .map(|line| line.chars().collect())
            .filter(|line: &Vec<char>| !line.iter().all(|c| SPACE.eq(c)))
            .collect();

        let width = grid[0].len();
        let start_idx = grid[0]
            .iter()
            .position(|&c| MANIFOLD.eq(&c))
            .expect("Manifold needs to be in first row.");

        let mut mask: Vec<u128> = vec![0; width];
        mask[start_idx] = 1;

        for line in grid {
            let mut next_counts: Vec<u128> = vec![0; width];

            for (i, &count) in mask.iter().enumerate() {
                if count == 0 { continue; }

                if SPLITTER.eq(&line[i]) {
                    if i > 0 { next_counts[i - 1] += count; }
                    if i + 1 < width { next_counts[i + 1] += count; }
                } else {
                    next_counts[i] += count;
                }
            }

            mask = next_counts;
        }

        let result: u128 = mask.iter().sum();
        assert_eq!(result, 4404709551015);
    }
}

// wincyj od 6156
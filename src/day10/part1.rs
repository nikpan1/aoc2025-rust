use std::fs::read_to_string;
use std::cmp::min;

#[derive(Debug)]
struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
}

impl Machine {
    pub fn solve(&self) -> Option<usize> {
        let (mut matrix, num_cols) = self.build_matrix();
        let pivots = self.gaussian_elimination(&mut matrix, num_cols);

        if self.is_inconsistent(&matrix, num_cols) {
            return None;
        }

        self.find_min_presses(&matrix, &pivots, num_cols)
    }

    fn build_matrix(&self) -> (Vec<Vec<bool>>, usize) {
        let rows = self.target_lights.len();
        let cols = self.buttons.len();
        let mut matrix = vec![vec![false; cols + 1]; rows];

        for (btn_idx, affected_lights) in self.buttons.iter().enumerate() {
            for &light_idx in affected_lights {
                if light_idx < rows {
                    matrix[light_idx][btn_idx] = true;
                }
            }
        }

        for (i, &is_on) in self.target_lights.iter().enumerate() {
            matrix[i][cols] = is_on;
        }

        (matrix, cols)
    }

    fn gaussian_elimination(&self, matrix: &mut Vec<Vec<bool>>, num_cols: usize) -> Vec<Option<usize>> {
        let num_rows = matrix.len();
        let mut pivot_row = 0;
        let mut pivots = vec![None; num_cols];

        for col in 0..num_cols {
            if pivot_row >= num_rows { break; }

            let mut found_row = None;
            for r in pivot_row..num_rows {
                if matrix[r][col] {
                    found_row = Some(r);
                    break;
                }
            }

            if let Some(r) = found_row {
                matrix.swap(pivot_row, r);
                pivots[col] = Some(pivot_row);

                for r_other in 0..num_rows {
                    if r_other != pivot_row && matrix[r_other][col] {
                        for c in col..=num_cols {
                            let val = matrix[pivot_row][c];
                            matrix[r_other][c] ^= val;
                        }
                    }
                }
                pivot_row += 1;
            }
        }
        pivots
    }

    fn is_inconsistent(&self, matrix: &Vec<Vec<bool>>, num_cols: usize) -> bool {
        let num_rows = matrix.len();
        for r in 0..num_rows {
            let row_is_zero = (0..num_cols).all(|c| !matrix[r][c]);
            if row_is_zero && matrix[r][num_cols] {
                return true;
            }
        }
        false
    }

    fn find_min_presses(&self, matrix: &Vec<Vec<bool>>, pivots: &Vec<Option<usize>>, num_cols: usize) -> Option<usize> {
        let free_vars: Vec<usize> = (0..num_cols)
            .filter(|&c| pivots[c].is_none())
            .collect();

        let mut min_presses = usize::MAX;
        let combinations = 1 << free_vars.len();
        for i in 0..combinations {
            let mut solution = vec![false; num_cols];

            for (idx, &col_idx) in free_vars.iter().enumerate() {
                if (i >> idx) & 1 == 1 {
                    solution[col_idx] = true;
                }
            }

            for col in (0..num_cols).rev() {
                if let Some(row) = pivots[col] {
                    let mut val = matrix[row][num_cols];
                    for c in (col + 1)..num_cols {
                        if matrix[row][c] && solution[c] {
                            val = !val;
                        }
                    }
                    solution[col] = val;
                }
            }

            let presses = solution.iter().filter(|&&b| b).count();
            min_presses = min(min_presses, presses);
        }

        if min_presses == usize::MAX { None } else { Some(min_presses) }
    }
}

fn parse_line(line: &str) -> Machine {
    let l_start = line.find('[').unwrap();
    let l_end = line.find(']').unwrap();
    let target_lights = line[l_start + 1..l_end]
        .chars()
        .map(|c| c == '#')
        .collect();

    let j_start = line.find('{').unwrap();
    let j_end = line.find('}').unwrap();

    let btn_str = &line[l_end + 1..j_start];
    let buttons = btn_str.split(')')
        .filter_map(|s| s.find('(').map(|i| &s[i + 1..]))
        .map(|s| s.split(',').map(|n| n.trim().parse().unwrap()).collect())
        .collect();

    Machine {
        target_lights,
        buttons,
    }
}

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

pub fn parse_input(input_file: &str) -> Vec<Machine> {
    read_file(input_file).lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_line)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let machines = parse_input("src/day10/input.txt");
        let mut total_presses = 0;

        for (i, m) in machines.iter().enumerate() {
            if let Some(presses) = m.solve() {
                total_presses += presses;
            }
        }
        assert_eq!(total_presses, 419);
    }
}

use std::fs::read_to_string;

#[derive(Debug)]
struct Machine {
    // Unused in Part 2 logic, kept for struct compatibility
    #[allow(dead_code)]
    target_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u128>,
}

impl Machine {
    pub fn solve(&self) -> Option<u128> {
        let rows = self.joltage.len();
        let cols = self.buttons.len();

        // Build Matrix (f64 for RREF)
        let mut matrix = vec![vec![0.0; cols + 1]; rows];

        for (btn_idx, affected_counters) in self.buttons.iter().enumerate() {
            for &counter_idx in affected_counters {
                if counter_idx < rows {
                    matrix[counter_idx][btn_idx] = 1.0;
                }
            }
        }

        for (i, &val) in self.joltage.iter().enumerate() {
            matrix[i][cols] = val as f64;
        }

        // 1. Gaussian Elimination
        let pivots = self.gaussian_elimination_rref(&mut matrix, rows, cols);

        // 2. Identify variables
        let mut free_vars = Vec::new();
        let mut pivot_vars = vec![None; rows];

        for col in 0..cols {
            if !pivots.contains(&Some(col)) {
                free_vars.push(col);
            }
        }
        for (row, &p_col) in pivots.iter().enumerate() {
            if let Some(c) = p_col {
                pivot_vars[row] = Some(c);
            }
        }

        // 3. Consistency Check
        for r in 0..rows {
            let is_zero_row = (0..cols).all(|c| matrix[r][c].abs() < 1e-9);
            if is_zero_row && matrix[r][cols].abs() > 1e-9 {
                return None;
            }
        }

        // 4. Calculate a safe upper bound for search.
        // A single button press contributes at least 1 to some counter.
        // Therefore, no button can be pressed more times than the maximum target joltage.
        let max_target = self.joltage.iter().max().copied().unwrap_or(100) as usize;

        let mut min_total = None;
        let mut current_free_vals = vec![0u128; free_vars.len()];

        self.search_free_vars(0, &free_vars, &mut current_free_vals, &matrix, &pivot_vars, cols, &mut min_total, max_target);

        min_total
    }

    fn gaussian_elimination_rref(&self, matrix: &mut Vec<Vec<f64>>, rows: usize, cols: usize) -> Vec<Option<usize>> {
        let mut pivot_row = 0;
        let mut pivots = vec![None; rows];

        for col in 0..cols {
            if pivot_row >= rows { break; }

            let mut found_row = None;
            for r in pivot_row..rows {
                if matrix[r][col].abs() > 1e-9 {
                    found_row = Some(r);
                    break;
                }
            }

            if let Some(r) = found_row {
                matrix.swap(pivot_row, r);
                pivots[pivot_row] = Some(col);

                let div = matrix[pivot_row][col];
                for c in col..=cols {
                    matrix[pivot_row][c] /= div;
                }

                for r_other in 0..rows {
                    if r_other != pivot_row {
                        let factor = matrix[r_other][col];
                        if factor.abs() > 1e-9 {
                            for c in col..=cols {
                                matrix[r_other][c] -= factor * matrix[pivot_row][c];
                            }
                        }
                    }
                }
                pivot_row += 1;
            }
        }
        pivots
    }

    fn search_free_vars(
        &self,
        idx: usize,
        free_vars: &Vec<usize>,
        free_vals: &mut Vec<u128>,
        matrix: &Vec<Vec<f64>>,
        pivot_vars: &Vec<Option<usize>>,
        num_cols: usize,
        min_total: &mut Option<u128>,
        limit: usize
    ) {
        // Optimization: if current partial sum already exceeds found minimum, abort branch
        let current_partial_sum: u128 = free_vals.iter().take(idx).sum();
        if let Some(m) = *min_total {
            if current_partial_sum >= m { return; }
        }

        if idx == free_vars.len() {
            let mut sum = current_partial_sum;

            // Check pivot variables determined by these free variables
            for (r, &p_opt) in pivot_vars.iter().enumerate() {
                if let Some(_) = p_opt {
                    let mut val = matrix[r][num_cols];
                    for (i, &f_col) in free_vars.iter().enumerate() {
                        val -= matrix[r][f_col] * (free_vals[i] as f64);
                    }

                    // Must be non-negative integer
                    if val < -1e-4 { return; }
                    let rounded = val.round();
                    if (val - rounded).abs() > 1e-4 { return; }

                    sum += rounded as u128;
                }
            }

            if min_total.map_or(true, |m| sum < m) {
                *min_total = Some(sum);
            }
            return;
        }

        // Search range expanded from 0..20 to 0..=limit
        for val in 0..=limit {
            free_vals[idx] = val as u128;
            self.search_free_vars(idx + 1, free_vars, free_vals, matrix, pivot_vars, num_cols, min_total, limit);
        }
    }
}

// --- Parsing (Identical) ---

fn parse_line(line: &str) -> Machine {
    let l_start = line.find('[').unwrap();
    let l_end = line.find(']').unwrap();
    let target_lights = line[l_start + 1..l_end]
        .chars()
        .map(|c| c == '#')
        .collect();

    let j_start = line.find('{').unwrap();
    let j_end = line.find('}').unwrap();
    let joltage = line[j_start + 1..j_end]
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    let btn_str = &line[l_end + 1..j_start];
    let buttons = btn_str
        .split(')')
        .filter_map(|s| s.find('(').map(|i| &s[i + 1..]))
        .map(|s| s.split(',').map(|n| n.trim().parse().unwrap()).collect())
        .collect();

    Machine {
        target_lights,
        buttons,
        joltage,
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
    fn test_part2() {
        let machines = parse_input("src/day10/input.txt");
        let mut total_presses = 0;
        let mut solved_count = 0;

        for m in machines.iter() {
            if let Some(presses) = m.solve() {
                total_presses += presses;
                solved_count += 1;
            }
        }
        println!("Solved {} machines. Total presses: {}", solved_count, total_presses);
    }
}
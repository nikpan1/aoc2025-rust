use std::fs::read_to_string;

pub const START_VALUE: i64 = 50;
pub const MAX_VALUE: i64 = 100;

struct Dial {
    pub current_position: i64,
}

fn parse(line: &str, direction: char) -> i64 {
    line.strip_prefix(direction)
        .expect("Given line expected to always have the specified prefix.")
        .parse::<i64>()
        .expect("Only acceptable values are integers.")
}

impl Dial {
    pub fn rotate(&mut self, vector: i64) -> u64 {
        let start_pos = self.current_position;
        let move_amount = vector.abs();
        let sum = self.current_position + vector;
        self.current_position = ((sum % MAX_VALUE) + MAX_VALUE) % MAX_VALUE;

        return if vector > 0 {
            (sum / MAX_VALUE) as u64
        } else {
            let dist_to_first_zero = if start_pos == 0 { MAX_VALUE } else { start_pos };
            let total_dist = move_amount - dist_to_first_zero;

            if total_dist >= 0 {
                1 + (total_dist / MAX_VALUE) as u64
            } else {
                0
            }
        }
    }

    pub fn rotate_directed(&mut self, vector: i64, direction: char) -> u64 {
        if direction == 'L' {
            self.rotate(-vector)
        } else {
            self.rotate(vector)
        }
    }
}

pub fn read_file(path: &str) -> String {
    read_to_string(path)
        .expect("Reading file failed.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let file_content = read_file("src/day1/input.txt");
        let mut dial: Dial = Dial {
            current_position: START_VALUE,
        };

        let mut result: u64 = 0;
        for line in file_content.lines() {
            let direction = line.chars().nth(0)
                .expect("Always R or L as the first char.");
            let value = parse(line, direction);
            result += dial.rotate_directed(value, direction);
        }

        assert_eq!(result, 6175);
    }
}
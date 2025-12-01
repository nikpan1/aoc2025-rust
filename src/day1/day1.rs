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
    pub fn rotate(&mut self, vector: i64) {
        let diff = self.current_position + vector;
        self.current_position = ((diff % MAX_VALUE) + MAX_VALUE) % MAX_VALUE;
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
    fn test_part1() {
        let file_content = read_file("src/day1/input.txt");
        let mut dial: Dial = Dial {
            current_position: START_VALUE,
        };

        let mut result: u64 = 0;
        for line in file_content.lines() {
            if line.contains('L') {
                let value = parse(line, 'L');
                dial.rotate(-value);
            } else {
                let value = parse(line, 'R');
                dial.rotate(value);
            }

            println!("{}", dial.current_position);
            if dial.current_position == 0 {
                result += 1;
            }
        }

        assert_eq!(result, 1102);
    }
}
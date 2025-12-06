use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut values: Vec<Vec<String>> = read_file("src/day6/input.txt")
            .split('\n')
            .map(|line| line.split_whitespace().map(|v| v.to_string()).collect())
            .collect();

        let operations = values.pop().unwrap();
        let size = values.iter().len();

        let mut final_result: u128 = 0;
        for (op_i, op) in operations.iter().enumerate() {
            let mut result: u128  = if op == "*" { 1 } else { 0 };

            for (i, entry) in values.iter().enumerate() {
                let value: u128  = entry[op_i].parse::<u128>().unwrap();
                if op == "*" {
                    result *= value;
                } else if op == "+" {
                    result += value;
                }
            }

            final_result += result;
        }

        assert_eq!(final_result, 6343365546996);
    }
}

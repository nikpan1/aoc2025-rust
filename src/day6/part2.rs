use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

#[derive(Clone)]
struct Number {
    number: u128,
    line_index: u8,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_part2() {
        let mut lines: Vec<String> = read_input_as_lines();
        let operators_line = lines.pop().unwrap();
        let operators: Vec<&str> = operators_line.split_whitespace().collect();

        let mut equations: HashMap<usize, HashMap<usize, Vec<Number>>> = HashMap::new();
        for (l_idx, line) in lines.iter().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            let mut op_idx = 0;
            for c_idx in 0..line.len() {
                let current = chars[c_idx];
                if current != ' ' {
                    equations.entry(op_idx)
                        .or_insert_with(HashMap::new)
                    .entry(c_idx)
                        .or_insert_with(Vec::new)
                    .push(Number {
                        number: current.to_digit(10).unwrap_or(0) as u128,
                        line_index: l_idx as u8,
                    });

                    if isEndOfNumber(line, &chars, c_idx) {
                        op_idx += 1;
                    }
                }
            }
        }
        let mut res = 0;
        for (op_idx, operator) in operators.iter().enumerate() {
            let equation = equations.get(&op_idx).unwrap();
            let numbers: Vec<u128> = equation.iter()
                .map(|(_, vec)| {
                    let mut ordered: Vec<Number> = (*vec).clone();
                    ordered.sort_by(|a, b| {a.line_index.cmp(&b.line_index)});

                    let size = ordered.len();
                    ordered.iter().enumerate()
                        .map(|(idx, value)| {
                            let power = (size - 1 - idx) as u32;
                            value.number * 10u128.pow(power)
                        })
                        .sum()
                }).collect();

            let equation_result = if *operator == "*" {
                let mut result = 1;
                for v in numbers {
                    result *= v;
                }
                result
            } else if *operator == "+" {
                let mut result = 0;
                for v in numbers {
                    result += v;
                }
                result
            } else { panic!("panic") };
            res += equation_result;
        }

        assert_eq!(res, 11136895955912);
    }

    fn isEndOfNumber(line: &String, chars: &Vec<char>, c_idx: usize) -> bool {
        c_idx < line.len() - 1 && *chars.get(c_idx + 1).unwrap() == ' '
    }

    fn read_input_as_lines() -> Vec<String> {
        read_file("src/day6/input.txt").lines()
            .map(|x| x.to_string())
            .collect()
    }
}

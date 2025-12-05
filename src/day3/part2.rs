use std::fs::read_to_string;
const KEEP_COUNT: usize = 12;

pub fn read_file(path: &str) -> String {
    read_to_string(path)
        .expect("Reading file failed.")
}

pub fn get_biggest_voltage(bank: &String) -> i128 {
    let nums_to_remove = bank.len() - KEEP_COUNT;
    let mut batteries: Vec<char> = Vec::new();

    let mut removed = 0;
    for digit in bank.chars() {
        while removed < nums_to_remove
            && !batteries.is_empty()
            && *batteries.last().unwrap() < digit
        {
            batteries.pop();
            removed += 1;
        }
        batteries.push(digit);
    }

    while removed < nums_to_remove {
        batteries.pop();
        removed += 1;
    }

    let result_string: String = batteries.iter().collect();
    result_string.parse::<i128>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let file_content = read_file("src/day3/input.txt");

        let voltages: i128 = file_content.lines()
            .map( |a| a.parse::<String>().unwrap())
            .map(|a| {
                let x = get_biggest_voltage(&a);
                println!("{} | {}", a, x);
                x
            })
            .sum();

        assert_eq!(voltages, 167549941654721);
    }
}
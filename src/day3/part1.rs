use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path)
        .expect("Reading file failed.")
}

pub fn get_biggest_voltage(bank: String) -> i128 {
    let mut batteries: Vec<i128> = bank.chars()
        .filter_map(|c| c.to_string().parse::<i128>().ok())
        .collect();

    let mut values_without_end: Vec<i128> = batteries.clone();
    values_without_end.pop();

    let max1 = find_max(&mut values_without_end);
    let id1 = find_first(&mut values_without_end, max1);

    let mut values = batteries.split_off(id1 + 1);
    let max2 = find_max(&mut values);

    max1 * 10 + max2
}

pub fn find_max(bank: &mut Vec<i128>) -> i128 {
    let max_index = bank.iter().enumerate()
        .max_by_key(|(_index, value)| *value)
        .map(|(index, _value)| index)
        .unwrap();

    bank.iter().nth(max_index).unwrap().clone()
}

pub fn find_first(bank: &mut Vec<i128>, value: i128) -> usize {
    for (c, i) in bank.iter().enumerate() {
        if *i == value {
            return c;
        }
    }

    panic!("Should not reach this place!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let file_content = read_file("src/day3/input.txt");

        let voltages: i128 = file_content.lines()
            .map( |a| a.parse().unwrap())
            .map(|a| get_biggest_voltage(a))
            .sum();

        assert_eq!(voltages, 16858);
    }
}
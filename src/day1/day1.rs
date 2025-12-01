use std::fs::read_to_string;

pub struct Pair {
    left: i32,
    right: i32,
}

pub fn read_file(path: &str) -> String {
    return read_to_string(path).expect("Reading file failed.");
}

fn parse_pair(c: &str) -> Pair {
    let mut parts = c.split_whitespace();

    let left = parts.next()
        .expect("Missing left number")
        .parse::<i32>()
        .expect("Invalid left number");

    let right = parts.next()
        .expect("Missing right number")
        .parse::<i32>()
        .expect("Invalid right number");

    (left, right)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_part1() {
        let file_content = read_file("src/day1/input.txt");

        let parsed_content: Vec<Pair> = file_content
            .lines()
            .map(|c| parse_pair(c))
            .collect();

        let mut left_values: Vec<i32> = parsed_content.iter()
            .map(|p| {p.left})
            .collect();

        let mut right_values: Vec<i32> = parsed_content.iter()
            .map(|p| {p.right})
            .collect();

        left_values.sort();
        right_values.sort();

        let result: i64 = left_values.iter()
            .zip(right_values.iter())
            .map(|(l, r)| (l - r).abs() as i64)
            .sum();

        print!("{}", result);
        assert_eq!(result, 1873376);
    }

    #[test]
    fn test_part2() {
        let file_content = read_file("src/day1/input.txt");

        let parsed_content: Vec<Pair> = file_content
            .lines()
            .map(|c| parse_pair(c))
            .collect();

        let mut left_values: Vec<i32> = parsed_content.iter()
            .map(|p| {p.left})
            .collect();

        let mut right_values: Vec<i32> = parsed_content.iter()
            .map(|p| {p.right})
            .collect();

        left_values.sort();
        let mut counter: HashMap<i32, u16> = HashMap::new();
        for val in left_values.iter() {
            let stat = counter.entry(*val).or_insert(0);
            *stat += 1;
        }

        let mut result: i64 = 0;
        for i in 0..right_values.len() {
            let val = right_values.get(i).unwrap();
            result += match counter.get(val) {
                Some(count) => (*val as i64) * (*count as i64),
                None =>  0,
            }
        }

        assert_eq!(result, 18997088);
    }
}
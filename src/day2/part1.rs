use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path)
        .expect("Reading file failed.")
}

struct Range {
    left_end: String,
    right_end: String,
}

impl Range {
    pub fn new(left: String, right: String) -> Self {
        Self {
            left_end: left,
            right_end: right,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.left_end.parse::<i128>().expect("") < self.right_end.parse::<i128>().expect("")
    }
}
struct Number {     // when we have number 1122
    left: i128,     // left = 11
    right: i128,    // right = 22
}

impl Number {
    pub fn is_less_or_equals(&self, other: &Number) -> bool {
        if self.left < other.left {
            return true;
        }
        if self.left > other.left {
            return false;
        }
        self.right <= other.right
    }

    pub fn is_in_range(&self, min: &Number, max: &Number) -> bool {
        min.is_less_or_equals(self) && self.is_less_or_equals(max)
    }

    pub fn increment(&mut self) {
        self.left += 1;
        self.right += 1;
    }
    pub fn get_total_value(&self) -> i128 {
        if self.right == 0 {
            return self.left * 10;
        }
        let digits = self.right.checked_ilog10().unwrap_or(0) + 1;
        let multiplier = 10_i128.pow(digits);
        self.left * multiplier + self.right
    }
}

fn parse_left(x: &String) -> String {
    if x.len() % 2 != 0 {
        let mut rounded: Vec<char> = "0".repeat(x.len() + 1).chars().collect();
        let target_idx = (rounded.len() + 1) / 2;
        rounded[0] = '1';
        rounded[target_idx] = '1';
        rounded.into_iter().collect()
    } else {
        x.clone()
    }
}
fn parse_right(y: &String) -> String {
    if y.len() % 2 != 0 {
        "9".repeat(y.len() - 1)
    } else {
        y.clone()
    }
}

impl Number {
    pub fn new(value: String) -> Self {
        Self{
            left: value[..value.len() / 2].parse().unwrap(),
            right : value[value.len() / 2 ..].parse().unwrap(),
        }
    }
    pub fn new_counter(value: String) -> Self {
        let mut left: i128= value[..value.len() / 2].parse().unwrap();
        let mut right: i128 = value[value.len() / 2 ..].parse().unwrap();
        if left < right {
            left += 1;
            right = left.clone();
        } else {
            right = left.clone()
        }
        Self{
            left,
            right,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::Split;
    use super::*;

    #[test]
    fn test_part1() {
        let file_content = read_file("src/day2/input.txt")
            .replace("\n", "");
        println!("Building ranges.");
        let ranges: Vec<Range> = file_content.split(',')
            .map(move |range| {
                let mut range_ends = range.split('-');
                Range {
                    left_end: consume_str_to_string(&mut range_ends),
                    right_end: consume_str_to_string(&mut range_ends),
                }
            })
            .collect();

        let parsed_ranges: Vec<Range> = ranges.iter()
            .map(|x| {
                let left = parse_left(&x.left_end);
                let right = parse_right(&x.right_end);

                println!("L: {} -> {}", x.left_end, left);
                println!("R: {} -> {}", x.right_end, right);
                Range {
                    left_end: left,
                    right_end: right
                }
            })
            .filter(Range::is_valid)
            .collect();
        println!("Invalid Ids.");
        let invalid_ids: Vec<i128> = parsed_ranges.iter()
            .flat_map(find_every_parities)
            .collect();

        let sum: i128 = invalid_ids.iter().sum();

        assert_eq!(sum, 56737989959);
    }

    fn consume_str_to_string(range_ends: &mut Split<char>) -> String {
        range_ends.nth(0)
            .expect("Should be 2 values.")
            .parse()
            .expect("Should be a string.")
    }

    fn find_every_parities(range: &Range) -> Vec<i128> {
        let mut counter = Number::new_counter(range.left_end.clone());
        let left_end = Number::new_counter(range.left_end.clone());
        let right_end = Number::new(range.right_end.clone());
        let mut result: Vec<i128> = Vec::new();
        println!("Finding for range <{}, {}>", range.left_end, range.right_end);
        while counter.is_in_range(&left_end, &right_end) {
            result.push(counter.get_total_value());
            println!("Z: {}", counter.get_total_value());
            counter.increment();
        }

        result
    }
}
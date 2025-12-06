use std::cmp::{max, min};
use std::fs::read_to_string;

#[derive(Debug)]
struct Range {
    left: u128,
    right: u128,
}
pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

pub fn are_overlapping(x: &Range, y: &Range) -> bool {
    x.left <= y.right && x.right >= y.left
}

pub fn fuse(x: &Range, y: &Range)  -> Range {
    Range {
        left: min(min(x.right, x.left), min(y.right, y.left)),
        right: max(max(x.right, x.left), max(y.right, y.left)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let sections = read_file("src/day5/input.txt");
        let mut sections = sections.split("\r\n\r\n");
        let ranges_block = sections.next().expect("Missing range section");
        let values_block = sections.next().expect("Missing values section");

        let mut ranges: Vec<Range> = ranges_block
            .lines()
            .map(|line| {
                let mut parts = line.split('-');
                Range {
                    left: parts.next().unwrap().parse().unwrap(),
                    right: parts.next().unwrap().parse().unwrap(),
                }
            })
            .collect();

        optimize(&mut ranges);

        let mut values: Vec<u128> = values_block
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<u128>().unwrap())
            .collect();

        values.sort();

        let mut range_iter = ranges.iter();
        let mut current_range = range_iter.next();
        let result: i128 = values.iter()
            .map(|number| {
                loop {
                    match current_range {
                        Some(T) => {
                            if number < &T.left {
                                return 0;
                            }
                            if &T.left <= number && number <= &T.right {
                                println!("Number: {}", number);
                                return 1;
                            }
                            current_range = range_iter.next();
                        }
                        None => { return 0; }
                    }
                }
            }).sum();

        println!("Ranges: {:?}", ranges);
        println!("Values: {:?}", values);

        assert_eq!(result, 798);
    }

    fn optimize(ranges: &mut Vec<Range>) {
        ranges.sort_by(|a, b| a.left.cmp(&b.left));

        let mut i = 0;
        while i + 1 < ranges.len() {
            if are_overlapping(&ranges[i], &ranges[i + 1]) {
                let new_range = fuse(&ranges[i], &ranges[i + 1]);
                ranges[i] = new_range;
                ranges.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }
}

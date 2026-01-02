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
    fn test_part2() {
        let ranges_block = read_file("src/day5/input.txt").replace("\r\n", "\n");
        let mut ranges_block = ranges_block.split("\n\n");
        let ranges_block = ranges_block.next().expect("Missing range section");

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

        let result: u128 = ranges.iter()
            .map(|range| range.right - range.left + 1)
            .sum();

        assert_eq!(result, 366181852921027);
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

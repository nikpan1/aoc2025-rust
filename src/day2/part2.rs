use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

struct Range {
    left_end_value: i128,
    right_end_value: i128,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::Split;

    #[test]
    fn test_part2() {
        let file_content = read_file("src/day2/input.txt").replace("\n", "");
        println!("Building ranges.");
        let ranges: Vec<Range> = file_content
            .split(',')
            .map(move |range| {
                let mut range_ends = range.split('-');
                let left = consume_str_to_string(&mut range_ends);
                let right = consume_str_to_string(&mut range_ends);
                let left_value = left.parse::<i128>().expect("Should be a number.");
                let right_value = right.parse::<i128>().expect("Should be a number.");
                Range {
                    left_end_value: left_value,
                    right_end_value: right_value,
                }
            })
            .collect();

        println!("Invalid Ids.");
        let invalid_ids: i128 = ranges.iter().flat_map(find_every_parities).sum();

        assert_eq!(invalid_ids, 79183223243);
    }
    fn consume_str_to_string(range_ends: &mut Split<char>) -> String {
        range_ends.nth(0).expect("Should be 2 values.").to_string()
    }

    fn find_every_parities(range: &Range) -> Vec<i128> {
        let mut result: Vec<i128> = Vec::new();
        for i in range.left_end_value..=range.right_end_value {
            if is_invalid_id(i) {
                result.push(i);
            }
        }
        result
    }

    fn is_invalid_id(id: i128) -> bool {
        let num = id.to_string();
        let total_len = num.len();
        let primes = find_all_divisors(total_len);

        primes.iter().any(|&amount_of_chunks| {
            if amount_of_chunks < 2 {
                return false;
            }

            let chunk_size = total_len / amount_of_chunks;
            let mut chunks = num.as_bytes().chunks(chunk_size);
            let first = chunks.next().expect("Always has at least one chunk");
            chunks.all(|c| c == first)
        })
    }

    fn find_all_divisors(size: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        let biggest_value = size.isqrt();
        for pivot in 2..=biggest_value {
            if size % pivot == 0 {
                result.push(pivot);
                let cofactor = size / pivot;
                if pivot != cofactor {
                    result.push(cofactor);
                }
            }
        }
        result.push(size);
        result
    }
}

use std::collections::HashMap;
use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

struct Device {
    name: String,
    outputs: Vec<String>,
}

fn count_paths(current: &str, graph: &HashMap<String, Vec<String>>, 
               memo: &mut HashMap<String, u64>) -> u64 {
    if current == "out" {
        return 1;
    }
    if let Some(&count) = memo.get(current) {
        return count;
    }

    let mut total = 0;
    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            total += count_paths(neighbor, graph, memo);
        }
    }

    memo.insert(current.to_string(), total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let graph: HashMap<String, Vec<String>> = read_file("src/day11/input.txt").lines()
            .map(|line| {
                let parts: Vec<&str> = line.split(": ").collect();
                let name = parts[0].to_string();
                let outputs = parts[1].split_whitespace().map(|s| s.to_string()).collect();
                (name, outputs)
            })
            .collect();

        let mut memo: HashMap<String, u64> = HashMap::new();
        let result = count_paths("you", &graph, &mut memo);

        assert_eq!(result, 690);
    }
}

use std::collections::HashMap;
use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

fn count_paths(current: &str, target: &str, graph: &HashMap<String, Vec<String>>,
               memo: &mut HashMap<String, u64>) -> u64 {
    if current == target {
        return 1;
    }
    if let Some(&count) = memo.get(current) {
        return count;
    }

    let mut total = 0;
    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            total += count_paths(neighbor, target, graph, memo);
        }
    }

    memo.insert(current.to_string(), total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let graph: HashMap<String, Vec<String>> = read_file("src/day11/input.txt").lines()
            .map(|line| {
                let parts: Vec<&str> = line.split(": ").collect();
                let name = parts[0].to_string();
                let outputs = parts[1].split_whitespace().map(|s| s.to_string()).collect();
                (name, outputs)
            })
            .collect();

        let run_count = |start: &str, end: &str| -> u64 {
            let mut memo = HashMap::new();
            count_paths(start, end, &graph, &mut memo)
        };

        let svr_dac = run_count("svr", "dac");
        let dac_fft = run_count("dac", "fft");
        let fft_out = run_count("fft", "out");
        let path1_total = svr_dac * dac_fft * fft_out;

        let svr_fft = run_count("svr", "fft");
        let fft_dac = run_count("fft", "dac");
        let dac_out = run_count("dac", "out");
        let path2_total = svr_fft * fft_dac * dac_out;

        let result = path1_total + path2_total;

        assert_eq!(result, 557332758684000);
    }
}
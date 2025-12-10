use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}
struct Coordinate {
    x: u128,
    y: u128,
    z: u128,
}

impl Coordinate {
    fn euclidean_distance(&self, other: &Coordinate) -> f64 {
        let dx = self.x.abs_diff(other.x) as f64;
        let dy = self.y.abs_diff(other.y) as f64;
        let dz = self.z.abs_diff(other.z) as f64;
        f64::sqrt(dx * dx + dy * dy + dz * dz)
    }
}

struct Connection {
    left: usize,
    right: usize,
    distance: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let positions: Vec<Coordinate> = read_file("src/day8/input.txt").lines()
            .map(|line| line.split(',').collect())
            .map(|line| {
                Coordinate {
                    x: parse(&line, 0),
                    y: parse(&line, 1),
                    z: parse(&line, 2),
                }
            })
            .collect();

        let size = positions.iter().len();
        let mut distance_matrix: Vec<Connection> = Vec::new();
        for left in 0..size {
            for right in (left + 1)..size {
                let distance = positions[left].euclidean_distance(&positions[right]);
                distance_matrix.push(Connection { left, right, distance })
            }
        }

        distance_matrix.sort_by(|x1, x2| x1.distance.total_cmp(&x2.distance));
        distance_matrix.truncate(1000);

        let mut parent: Vec<usize> = (0..size).collect();
        fn find(parent: &mut Vec<usize>, i: usize) -> usize {
            if parent[i] == i {
                i
            } else {
                let root = find(parent, parent[i]);
                parent[i] = root;
                root
            }
        }

        for connection in distance_matrix {
            let root_a = find(&mut parent, connection.left);
            let root_b = find(&mut parent, connection.right);

            if root_a != root_b {
                parent[root_a] = root_b;
            }
        }

        // We use a HashMap to count how many nodes point to each root
        use std::collections::HashMap;
        let mut counts: HashMap<usize, u64> = HashMap::new();

        for i in 0..size {
            let root = find(&mut parent, i);
            *counts.entry(root).or_insert(0) += 1;
        }

        let mut sizes: Vec<u64> = counts.values().cloned().collect();
        sizes.sort_by(|a, b| b.cmp(a));
        let result: u64 = sizes.iter().take(3).product();

        assert_eq!(result, 40);
    }

    fn parse(values: &Vec<&str>, index: usize) -> u128 {
        values.get(index).unwrap().parse::<u128>().unwrap()
    }
}

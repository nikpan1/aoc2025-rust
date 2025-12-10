use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

fn parse(values: &Vec<&str>, index: usize) -> i64 {
    values.get(index).unwrap().parse::<i64>().unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinate {
    fn euclidean_distance(&self, other: &Coordinate) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
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
    fn test_part2() {
        let positions: Vec<Coordinate> = crate::day8::part1::read_file("src/day8/input.txt").lines()
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

        let mut groups: Vec<usize> = (0..size).collect();
        let mut connections_found = 0;

        let mut result: i64 = 0;
        for conn in distance_matrix {
            let group_left = groups[conn.left];
            let group_right = groups[conn.right];

            if group_left != group_right {
                connections_found += 1;
                for i in 0..size {
                    if groups[i] == group_right {
                        groups[i] = group_left;
                    }
                }
                if connections_found == size - 1 {
                    result = positions[conn.left].x * positions[conn.right].x
                }
            }
        }

        assert_eq!(result, 42047840);
    }
}
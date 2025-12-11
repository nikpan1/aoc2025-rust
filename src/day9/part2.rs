use std::cmp::{max, min};
use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

struct Coordinate {
    x: u128,
    y: u128,
}

impl Coordinate {
    fn manhattan_distance(&self, other: &Coordinate) -> f64 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        (dx + dy) as f64
    }

    fn field(&self, other: &Coordinate) -> u128 {
        let dx = max(self.x.abs_diff(other.x) + 1, 1);
        let dy = max(self.y.abs_diff(other.y) + 1, 1);
        return dx * dy as u128
    }
}

struct Connection {
    left: usize,
    right: usize,
    field: u128,
}

impl Connection {
    pub fn is_inside(&self, pos: &Coordinate, positions: &[Coordinate]) -> bool {
        let p1 = &positions[self.left];
        let p2 = &positions[self.right];

        let min_x = min(p1.x, p2.x);
        let max_x = max(p1.x, p2.x);
        let min_y = min(p1.y, p2.y);
        let max_y = max(p1.y, p2.y);

        pos.x > min_x && pos.x < max_x && pos.y > min_y && pos.y < max_y
    }
}

fn parse(values: &Vec<&str>, index: usize) -> u128 {
    values.get(index).unwrap().parse::<u128>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let positions: Vec<Coordinate> = read_file("src/day9/input.txt").lines()
            .map(|line| line.split(',').collect())
            .map(|line| {
                Coordinate {
                    x: parse(&line, 0),
                    y: parse(&line, 1),
                }
            })
            .collect();

        let size = positions.iter().len();
        let mut distance_matrix: Vec<Connection> = Vec::new();
        for left in 0..size {
            for right in (left + 1)..size {
                let field = positions[left].field(&positions[right]);
                distance_matrix.push(Connection { left, right, field })
            }
        }

        distance_matrix.sort_by(|x1, x2| x1.field.cmp(&x2.field));
        let mut max_con = distance_matrix.pop().unwrap();

        while positions.iter().any(|pos| max_con.is_inside(pos, &positions)) {
            max_con = distance_matrix.pop().unwrap();
        }
        assert_eq!(max_con.field, 4777409595);
    }
}

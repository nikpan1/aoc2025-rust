use std::cmp::{min, max};
use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

struct Coordinate {
    x: u128,
    y: u128,
}

impl Coordinate {
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

fn parse(values: &Vec<&str>, index: usize) -> u128 {
    values.get(index).unwrap().parse::<u128>().unwrap()
}

fn is_rect_inside(poly: &Vec<Coordinate>, rect: &Vec<Coordinate>) -> bool {
    let r1 = &rect[0];
    let r2 = &rect[1];

    let min_x = min(r1.x, r2.x);
    let max_x = max(r1.x, r2.x);
    let min_y = min(r1.y, r2.y);
    let max_y = max(r1.y, r2.y);

    if edges_intersect_rect(poly, min_x, max_x, min_y, max_y) {
        return false;
    }

    let mid_x = (min_x as f64 + max_x as f64) / 2.0;
    let mid_y = (min_y as f64 + max_y as f64) / 2.0;

    is_point_in_poly(poly, mid_x, mid_y)
}

fn edges_intersect_rect(poly: &Vec<Coordinate>, min_x: u128, max_x: u128, min_y: u128, max_y: u128) -> bool {
    let n = poly.len();
    for i in 0..n {
        let p1 = &poly[i];
        let p2 = &poly[(i + 1) % n];

        if p1.x == p2.x { // Vertical Edge
            let edge_x = p1.x;
            let edge_y_min = min(p1.y, p2.y);
            let edge_y_max = max(p1.y, p2.y);

            // Edge is strictly between rect X bounds AND overlaps Y bounds
            if edge_x > min_x && edge_x < max_x && edge_y_max > min_y && edge_y_min < max_y {
                return true;
            }
        } else { // Horizontal Edge
            let edge_y = p1.y;
            let edge_x_min = min(p1.x, p2.x);
            let edge_x_max = max(p1.x, p2.x);

            // Edge is strictly between rect Y bounds AND overlaps X bounds
            if edge_y > min_y && edge_y < max_y && edge_x_max > min_x && edge_x_min < max_x {
                return true;
            }
        }
    }
    false
}

fn is_point_in_poly(poly: &Vec<Coordinate>, x: f64, y: f64) -> bool {
    let mut inside = false;
    let n = poly.len();

    for i in 0..n {
        let p1 = &poly[i];
        let p2 = &poly[(i + 1) % n];

        let p1x = p1.x as f64;
        let p1y = p1.y as f64;
        let p2x = p2.x as f64;
        let p2y = p2.y as f64;

        // Check if ray crosses the segment
        let intersect = ((p1y > y) != (p2y > y))
            && (x < (p2x - p1x) * (y - p1y) / (p2y - p1y) + p1x);

        if intersect {
            inside = !inside;
        }
    }
    inside
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let positions: Vec<Coordinate> = read_file("src/day9/input.txt").lines()
            .map(|line| line.split(',').collect::<Vec<&str>>())
            .map(|line| {
                Coordinate {
                    x: parse(&line, 0),
                    y: parse(&line, 1),
                }
            })
            .collect();

        let mut connections: Vec<Connection> = Vec::new();
        let size = positions.len();
        for i in 0..size {
            for j in (i + 1)..size {
                let area = positions[i].field(&positions[j]);
                connections.push(Connection { left: i, right: j, field: area });
            }
        }

        connections.sort_by(|a, b| b.field.cmp(&a.field));

        for conn in connections {
            let p1 = &positions[conn.left];
            let p2 = &positions[conn.right];
            let rect = vec![
                Coordinate { x: p1.x, y: p1.y },
                Coordinate { x: p2.x, y: p2.y }
            ];

            if is_rect_inside(&positions, &rect) {
                assert_eq!(conn.field, 1473551379);
                return;
            }
        }


    }
}

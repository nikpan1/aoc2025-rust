use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    r: i32,
    c: i32,
}

struct Shape {
    variants: Vec<Vec<Point>>,
    size: usize,
}

struct Region {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

impl Point {
    fn rotate(&self) -> Point {
        Point {
            r: self.c,
            c: -self.r,
        }
    }

    fn flip(&self) -> Point {
        Point {
            r: self.r,
            c: -self.c,
        }
    }

    fn normalize(points: &[Point]) -> Vec<Point> {
        if points.is_empty() {
            return vec![];
        }
        let mut p = points.to_vec();
        p.sort();
        let min_r = p[0].r;
        let min_c = p[0].c;
        p.iter()
            .map(|pt| Point {
                r: pt.r - min_r,
                c: pt.c - min_c,
            })
            .collect()
    }
}

impl Shape {
    fn new(base_points: Vec<Point>) -> Self {
        let size = base_points.len();
        let variants = Shape::generate_variants(&base_points);
        Shape { variants, size }
    }

    fn new_gap() -> Self {
        Shape {
            variants: vec![vec![Point { r: 0, c: 0 }]],
            size: 1,
        }
    }

    fn generate_variants(base: &[Point]) -> Vec<Vec<Point>> {
        let mut unique = HashSet::new();
        let mut current = base.to_vec();

        for _ in 0..4 {
            unique.insert(Point::normalize(&current));
            current = current.iter().map(|p| p.rotate()).collect();
        }

        current = base.iter().map(|p| p.flip()).collect();

        for _ in 0..4 {
            unique.insert(Point::normalize(&current));
            current = current.iter().map(|p| p.rotate()).collect();
        }

        let mut result: Vec<Vec<Point>> = unique.into_iter().collect();
        result.sort();
        result
    }
}

fn is_solvable(
    grid: &mut Vec<bool>,
    w: usize,
    h: usize,
    counts: &mut Vec<usize>,
    shapes: &[Shape],
) -> bool {
    let next_empty = grid.iter().position(|&x| !x);

    match next_empty {
        None => true,
        Some(idx) => {
            let r = (idx / w) as i32;
            let c = (idx % w) as i32;

            for id in 0..counts.len() {
                if counts[id] > 0 {
                    counts[id] -= 1;

                    for variant in &shapes[id].variants {
                        if can_place(grid, w, h, r, c, variant) {
                            place(grid, w, r, c, variant, true);

                            if is_solvable(grid, w, h, counts, shapes) {
                                return true;
                            }

                            place(grid, w, r, c, variant, false);
                        }
                    }
                    counts[id] += 1;
                }
            }
            false
        }
    }
}

fn can_place(grid: &[bool], w: usize, h: usize, r: i32, c: i32, variant: &[Point]) -> bool {
    for p in variant {
        let nr = r + p.r;
        let nc = c + p.c;
        if nr < 0
            || nr >= h as i32
            || nc < 0
            || nc >= w as i32
            || grid[(nr as usize) * w + (nc as usize)]
        {
            return false;
        }
    }
    true
}

fn place(grid: &mut [bool], w: usize, r: i32, c: i32, variant: &[Point], value: bool) {
    for p in variant {
        let idx = ((r + p.r) as usize) * w + ((c + p.c) as usize);
        grid[idx] = value;
    }
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let lines: Vec<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    let mut raw_shapes: HashMap<usize, Vec<Point>> = HashMap::new();
    let mut regions: Vec<Region> = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        if line.contains('x') && line.contains(':') {
            regions.push(parse_region(line));
            i += 1;
        } else if line.ends_with(':') {
            let (id, points, next_i) = parse_shape(&lines, i);
            raw_shapes.insert(id, Point::normalize(&points));
            i = next_i;
        } else {
            i += 1;
        }
    }

    let max_id = *raw_shapes.keys().max().unwrap_or(&0);
    let mut shapes = Vec::with_capacity(max_id + 2);

    for id in 0..=max_id {
        if let Some(points) = raw_shapes.get(&id) {
            shapes.push(Shape::new(points.clone()));
        } else {
            shapes.push(Shape::new(vec![]));
        }
    }

    shapes.push(Shape::new_gap());

    (shapes, regions)
}

fn parse_region(line: &str) -> Region {
    let parts: Vec<&str> = line.split(':').collect();
    let dims: Vec<usize> = parts[0].split('x').map(|x| x.parse().unwrap()).collect();
    let counts: Vec<usize> = parts[1]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    Region {
        width: dims[0],
        height: dims[1],
        counts,
    }
}

fn parse_shape(lines: &[&str], start_index: usize) -> (usize, Vec<Point>, usize) {
    let id = lines[start_index]
        .trim_end_matches(':')
        .parse::<usize>()
        .unwrap();
    let mut i = start_index + 1;
    let mut points = Vec::new();
    let mut r = 0;

    while i < lines.len() && !lines[i].contains(':') {
        for (c, ch) in lines[i].chars().enumerate() {
            if ch == '#' {
                points.push(Point { r, c: c as i32 });
            }
        }
        r += 1;
        i += 1;
    }

    (id, points, i)
}

fn is_region_prepared(region: &mut Region, shapes: &[Shape]) -> bool {
    let presents_area: usize = region
        .counts
        .iter()
        .enumerate()
        .map(|(id, c)| c * shapes[id].size)
        .sum();

    let total_area = region.width * region.height;

    if presents_area > total_area {
        return false;
    }

    let gap_count = total_area - presents_area;
    region.counts.push(gap_count);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_file("src/day12/input.txt");
        let (shapes, regions) = parse_input(&input);

        let mut solvable_count = 0;

        for mut region in regions {
            if !is_region_prepared(&mut region, &shapes) {
                continue;
            }

            let mut grid = vec![false; region.width * region.height];

            if is_solvable(&mut grid, region.width, region.height, &mut region.counts, &shapes) {
                solvable_count += 1;
            }
        }

        assert_eq!(solvable_count, 479);
    }
}

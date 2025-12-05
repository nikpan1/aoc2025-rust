use std::fs::read_to_string;
type Grid<T> = Vec<Vec<T>>;

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn test_part2() {
        let file_content = read_file("src/day4/input.txt");
        let lines: Vec<&str> = file_content.lines().collect();

        let rows = lines.len();
        let cols = lines[0].len();

        let mut is_paper: Grid<bool> = vec![vec![false; cols + 2]; rows + 2];
        let mut neighbors: Grid<i8> = vec![vec![0; cols + 2]; rows + 2];

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '@' {
                    is_paper[y + 1][x + 1] = true;
                    explode(&mut neighbors, y + 1, x + 1, 1);
                }
            }
        }

        let mut queue = VecDeque::new();
        for y in 1..=rows {
            for x in 1..=cols {
                if is_paper[y][x] && neighbors[y][x] < 4 {
                    queue.push_back((y, x));
                    is_paper[y][x] = false;
                }
            }
        }

        let mut removed_count = 0;
        while let Some((y, x)) = queue.pop_front() {
            removed_count += 1;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }

                    let ny = (y as isize + dy) as usize;
                    let nx = (x as isize + dx) as usize;

                    neighbors[ny][nx] -= 1;

                    if is_paper[ny][nx] && neighbors[ny][nx] < 4 {
                        is_paper[ny][nx] = false;
                        queue.push_back((ny, nx));
                    }
                }
            }
        }

        assert_eq!(removed_count, 8484);
    }
    
    fn explode(grid: &mut Grid<i8>, y: usize, x: usize, v: i8) {
        grid[y - 1][x - 1] += v;
        grid[y - 1][x - 0] += v;
        grid[y - 1][x + 1] += v;
        grid[y - 0][x + 1] += v;
        grid[y - 0][x - 1] += v;
        grid[y + 1][x - 1] += v;
        grid[y + 1][x - 0] += v;
        grid[y + 1][x + 1] += v;
    }
}

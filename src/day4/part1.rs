use std::fs::read_to_string;
type Grid<T> = Vec<Vec<T>>;

pub fn read_file(path: &str) -> String {
    read_to_string(path)
        .expect("Reading file failed.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let file_content: Vec<String> = read_file("src/day4/input.txt")
            .lines()
            .map(|x| x.to_string())
            .collect();

        let cols_size: usize = file_content.len()  + 2;
        let rows_size: usize = file_content.get(0).unwrap().len() + 2;

        let mut grid: Grid<String> = vec![vec![".".to_string(); cols_size]; rows_size];
        let mut grid_counter: Grid<u16> = vec![vec![0; cols_size]; rows_size];

        for y in 1..(rows_size - 1) {
            for x in 1..(cols_size - 1) {
                grid[y][x] = file_content
                    .get(y - 1).unwrap()
                    .chars().nth(x - 1).unwrap()
                    .to_string();
                if grid[y][x] == "@" {
                    explode(&mut grid_counter, y, x);
                }
            }
        }

        let mut result: u16 = 0;
        for y in 1..(rows_size - 1) {
            for x in 1..(cols_size - 1) {
                if grid_counter[y][x] < 4 && grid[y][x] == "@" {
                    result += 1;
                }
            }
        }
        assert_eq!(result, 1467);
    }

    fn explode(grid: &mut Grid<u16>, y: usize, x: usize) {
        grid[y - 1][x - 1] += 1;
        grid[y - 1][x - 0] += 1;
        grid[y - 1][x + 1] += 1;
        grid[y - 0][x + 1] += 1;
        grid[y - 0][x - 1] += 1;
        grid[y + 1][x - 1] += 1;
        grid[y + 1][x - 0] += 1;
        grid[y + 1][x + 1] += 1;
    }
}
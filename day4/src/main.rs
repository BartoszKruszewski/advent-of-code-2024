use std::fs;
use std::io::{self, BufRead};

fn find_xmas(grid: &[Vec<char>]) -> usize {
    let mut count = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if grid[row][col] == 'A' {
                let tl = grid[row - 1][col - 1];
                let tr = grid[row - 1][col + 1];
                let bl = grid[row + 1][col - 1];
                let br = grid[row + 1][col + 1];
                let diag1 = (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M');
                let diag2 = (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M');
                if diag1 && diag2 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let file_path = "input.txt";
    let file = fs::File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        grid.push(line.chars().collect::<Vec<_>>());
    }

    let count = find_xmas(&grid);
    println!("{}", count);
}

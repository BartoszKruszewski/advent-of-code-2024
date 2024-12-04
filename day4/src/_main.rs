use std::fs;
use std::io::{self, BufRead};

fn find_xmas(grid: &[Vec<char>]) -> usize {
    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // diagonal down-right
        (1, -1),  // diagonal down-left
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // diagonal up-left
        (-1, 1),  // diagonal up-right
    ];

    let word = "XMAS";
    let word_len = word.len();
    let mut count = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &directions {
                let mut matched = true;

                for (i, ch) in word.chars().enumerate() {
                    let r = row as isize + dr * i as isize;
                    let c = col as isize + dc * i as isize;

                    if r < 0 || r >= rows as isize || c < 0 || c >= cols as isize {
                        matched = false;
                        break;
                    }

                    if grid[r as usize][c as usize] != ch {
                        matched = false;
                        break;
                    }
                }

                if matched {
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
    println!("{}}", count);
}

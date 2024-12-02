use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_safe(levels: &[i32]) -> bool {
    let differences: Vec<i32> = levels.windows(2).map(|w| w[1] - w[0]).collect();
    let increasing = differences.iter().all(|&diff| diff >= 1 && diff <= 3);
    let decreasing = differences.iter().all(|&diff| diff <= -1 && diff >= -3);
    increasing || decreasing
}

fn is_safe_with_dampener(levels: &[i32]) -> bool {
    for i in 0..levels.len() {
        let mut modified_levels = levels.to_vec();
        modified_levels.remove(i);
        if is_safe(&modified_levels) {
            return true;
        }
    }
    false
}

fn count_safe_reports_with_dampener(reports: Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|report| is_safe(report) || is_safe_with_dampener(report)).count()
}

fn read_reports_from_file(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let reports: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    Ok(reports)
}

fn main() -> io::Result<()> {
    let filename = "input.txt";
    let reports = read_reports_from_file(filename)?;

    let safe_reports_count = count_safe_reports_with_dampener(reports);
    println!("{}", safe_reports_count);

    Ok(())
}

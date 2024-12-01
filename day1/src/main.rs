use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let (left, right) = input
        .split_whitespace()
        .enumerate()
        .partition::<Vec<_>, _>(|&(i, _)| i % 2 == 0);

    let mut left: Vec<usize> = left.iter().map(|(_, v)| v.parse::<usize>().unwrap()).collect();
    let mut right: Vec<usize> = right.iter().map(|(_, v)| v.parse::<usize>().unwrap()).collect();

    left.sort();
    right.sort();
    
    let res1 = left
        .iter()
        .zip(right.iter())
        .fold(0usize, |a, (l , &r)| a + l.abs_diff(r));

    let mut counts = vec![0usize; 100_000];
    for number in right {
        counts[number] += 1;
    }

    let res2= left
        .iter()
        .fold(0usize, |a, &l| a + l * counts[l]);

    println!("{res1}");
    println!("{res2}");

    Ok(())
}

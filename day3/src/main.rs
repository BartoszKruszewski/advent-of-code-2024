use regex::Regex;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let control_re = Regex::new(r"(do\(\)|don't\(\))").unwrap();

    let mut sum = 0;
    let mut enabled = true;

    let mut pos = 0;
    while pos < input.len() {
        if let Some(mul_match) = mul_re.find(&input[pos..]) {
            let mul_start = pos + mul_match.start();
            let mul_end = pos + mul_match.end();

            if let Some(control_match) = control_re.find(&input[pos..mul_start]) {
                let control_text = &input[pos + control_match.start()..pos + control_match.end()];
                enabled = control_text == "do()";
            }

            if enabled {
                let captures = mul_re.captures(&input[mul_start..mul_end]).unwrap();
                let x: i32 = captures[1].parse().unwrap();
                let y: i32 = captures[2].parse().unwrap();
                sum += x * y;
            }

            pos = mul_end;
        } else if let Some(control_match) = control_re.find(&input[pos..]) {
            let control_start = pos + control_match.start();
            let control_end = pos + control_match.end();
            let control_text = &input[control_start..control_end];
            enabled = control_text == "do()";
            pos = control_end;
        } else {
            break;
        }
    }

    println!("{sum}");
    Ok(())
}
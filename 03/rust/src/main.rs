use std::io::BufRead;
use std::{fs::File, io};

fn read_file_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;

    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect())
}

fn main() {
    let lines = read_file_lines("data.txt").expect("Sorry, can't read this file");

    let joltage: u128 = lines
        .iter()
        .map(|line| {
            let digits: Vec<u128> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u128)
                .collect();

            let mut max_joltage = 0;

            for i in 0..digits.len() - 1 {
                for j in i + 1..digits.len() {
                    let joltage = digits[i] * 10 + digits[j];
                    if joltage > max_joltage {
                        max_joltage = joltage;
                    }
                }
            }

            max_joltage
        })
        .sum();

    println!("Part1: Total joltage is: {}", joltage);

    let joltage_2: u128 = lines
        .iter()
        .map(|line| {
            let digits: Vec<u128> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u128)
                .collect();

            let k = 12;
            let mut stack = Vec::new();
            let mut to_remove = digits.len().saturating_sub(k);

            for &d in &digits {
                while let Some(&last) = stack.last() {
                    if last < d && to_remove > 0 {
                        stack.pop();
                        to_remove -= 1;
                    } else {
                        break;
                    }
                }
                stack.push(d);
            }

            let largest = stack.into_iter().take(k);

            largest.fold(0u128, |acc, d| acc * 10 + d)
        })
        .sum();

    println!("Part2: Total joltage is: {}", joltage_2);
}

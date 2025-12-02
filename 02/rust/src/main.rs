use std::io::BufRead;
use std::{fs::File, io};

fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;

    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect())
}

fn main() {
    let file_lines = read_lines("data.txt").expect("Error reading file!");

    let invalid_ids_sum: u128 = file_lines
        .iter()
        .flat_map(|line| line.split(","))
        .map(String::from)
        .flat_map(|range| {
            let mut parts = range.split("-");
            let part1 = parts.next().unwrap_or("").parse::<u128>().unwrap_or(0);
            let part2 = parts.next().unwrap_or("").parse::<u128>().unwrap_or(0);

            (part1..=part2)
                .filter(|&code| {
                    let s = code.to_string();
                    let mid = s.len() / 2;
                    let first_half = &s[..mid];
                    let second_half = &s[mid..];
                    first_half == second_half
                })
                .collect::<Vec<u128>>()
        })
        .sum();

    println!(
        "Part 1: Adding up all invalid ids gives me: {}",
        invalid_ids_sum
    );

    let invalid_ids_sum_2: u128 = file_lines
        .iter()
        .flat_map(|line| line.split(","))
        .map(String::from)
        .flat_map(|range| {
            let mut parts = range.split("-");
            let part1 = parts.next().unwrap_or("").parse::<u128>().unwrap_or(0);
            let part2 = parts.next().unwrap_or("").parse::<u128>().unwrap_or(0);

            (part1..=part2)
                .filter(|&code| {
                    let s = code.to_string();
                    let len = s.len();

                    for l in 1..=(len / 2) {
                        if len % l != 0 {
                            continue;
                        }

                        let sub = &s[..l];
                        let repeated = sub.repeat(len / l);

                        if repeated == s {
                            return true;
                        }
                    }

                    false
                })
                .collect::<Vec<u128>>()
        })
        .sum();

    println!(
        "Part 2: Adding up all invalid ids gives me: {}",
        invalid_ids_sum_2
    );
}

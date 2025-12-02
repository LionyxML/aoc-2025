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

    let total_pos = 100;
    let initial_pos = 50;

    // Count how many times it stopped on 0
    let zero_hits = file_lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (dir, num) = line.split_at(1);
            let n: i32 = num.parse().unwrap();
            (dir, n)
        })
        .fold((initial_pos, 0), |(pos, count), (dir, n)| {
            let new_pos = match dir {
                "R" => (pos + n) % total_pos,
                "L" => (pos - n + total_pos) % total_pos,
                _ => panic!("Invalid direction!"),
            };
            (new_pos, count + if new_pos == 0 { 1 } else { 0 })
        })
        .1;

    println!("1.) Number of times dial points at 0: {}", zero_hits);

    // Count how many times it passed through 0
    let zero_hits_2 = file_lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (dir, num) = line.split_at(1);
            let n: i32 = num.parse().unwrap();
            (dir, n)
        })
        .fold((initial_pos, 0), |(pos, count), (dir, n)| {
            let new_pos = match dir {
                "R" => (pos + n) % total_pos,
                "L" => (pos - n + total_pos) % total_pos,
                _ => panic!("Invalid direction!"),
            };

            let crossed_zeros = match dir {
                "R" => ((pos + 1)..=pos + n)
                    .filter(|&p| p % total_pos == 0)
                    .count() as i32,
                "L" => ((pos - n)..=pos - 1)
                    .filter(|&p| (p + total_pos) % total_pos == 0)
                    .count() as i32,
                _ => 0,
            };

            (new_pos, count + crossed_zeros)
        })
        .1;

    println!("2.) Number of passes through 0      : {}", zero_hits_2);
}

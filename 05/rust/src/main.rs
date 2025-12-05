use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(path: &str) -> Result<Vec<String>, io::Error> {
    BufReader::new(File::open(path)?).lines().collect()
}

fn main() {
    let file_lines = read_file("data.txt").expect("Cannot read file!");

    // PART 1

    let mut id_ranges = vec![];
    let mut ingredients = vec![];

    for line in file_lines {
        if let Some((a, b)) = line.split_once("-") {
            id_ranges.push((a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        } else {
            if line.len() > 0 {
                ingredients.push(line.parse::<i64>().unwrap())
            }
        }
    }

    let mut fresh: HashSet<i64> = HashSet::new();

    for ingredient in &ingredients {
        for id_range in &id_ranges {
            if ingredient >= &id_range.0 && ingredient <= &id_range.1 {
                fresh.insert(*ingredient);
            }
        }
    }

    println!("Part1: fresh items: {}", fresh.len());

    // PART 2
    id_ranges.sort_by_key(|r| r.0);

    let mut merged = Vec::<(i64, i64)>::new();

    for (start, end) in id_ranges {
        if let Some((_last_start, last_end)) = merged.last_mut() {
            if start <= *last_end + 1 {
                *last_end = (*last_end).max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    let mut count = 0i64;
    for (start, end) in merged {
        count += end - start + 1;
    }

    println!("Part2: fresh items: {}", count);
}

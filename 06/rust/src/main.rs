use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    BufReader::new(File::open(path)?).lines().collect()
}

#[derive(Debug, Clone)]
enum Cell {
    Num(u128),
    Sym(String),
}

fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = v.len();
    let cols = v[0].len(); // NOTE: assume all rows same length

    let mut out = vec![Vec::with_capacity(rows); cols];

    for r in 0..rows {
        for c in 0..cols {
            out[c].push(v[r][c].clone());
        }
    }

    out
}

fn main() {
    let file_lines = read_lines("data.txt").expect("Cannot read file!");

    // PART 1
    let mut data: Vec<Vec<Cell>> = file_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|tok| {
                    if let Ok(n) = tok.parse::<u128>() {
                        Cell::Num(n)
                    } else {
                        Cell::Sym(tok.to_string())
                    }
                })
                .collect()
        })
        .collect();

    let operations = data.pop().unwrap();
    let numbers = transpose(data);

    let mut all_problems_sum = 0u128;

    for (idx, op) in operations.iter().enumerate() {
        match op {
            Cell::Sym(s) if s == "+" => {
                let sum: u128 = numbers[idx]
                    .iter()
                    .map(|cell| match cell {
                        Cell::Num(n) => *n as u128,
                        _ => 0,
                    })
                    .sum();

                all_problems_sum += sum;
            }

            Cell::Sym(s) if s == "*" => {
                let prod: u128 = numbers[idx]
                    .iter()
                    .map(|cell| match cell {
                        Cell::Num(n) => *n as u128,
                        _ => 1,
                    })
                    .product();

                all_problems_sum += prod;
            }

            _ => {}
        }
    }

    println!(
        "Part1: The total sum of the individual problems is: {:?}",
        all_problems_sum
    );

    // PART 2
    let lines: Vec<Vec<char>> = file_lines.iter().map(|l| l.chars().collect()).collect();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let height = lines.len();

    let mut columns: Vec<String> = Vec::new();
    for x in (0..width).rev() {
        let mut col = String::with_capacity(height);
        for row in &lines {
            col.push(row.get(x).copied().unwrap_or(' '));
        }
        columns.push(col);
    }

    let cleaned_cols: Vec<String> = columns
        .iter()
        .map(|c| {
            c.chars()
                .map(|ch| {
                    if ch.is_ascii_digit() {
                        ch
                    } else {
                        ' ' // convert anything weird into a space
                    }
                })
                .collect::<String>()
                .trim()
                .to_string()
        })
        .collect();

    let joined = cleaned_cols.join(" ");

    let groups = joined.split("  ");

    let mut total2: u128 = 0;

    for (i, group) in groups.enumerate() {
        let op = match &operations[operations.len() - 1 - i] {
            Cell::Sym(s) => s.as_str(),
            _ => panic!("Expected operator"),
        };

        let nums: Vec<u128> = group
            .split_whitespace()
            .filter(|tok| !tok.is_empty())
            .map(|tok| tok.parse::<u128>().unwrap())
            .collect();

        let val = match op {
            "*" => nums.iter().product::<u128>(),
            _ => nums.iter().sum::<u128>(),
        };

        total2 += val;
    }

    println!("Part2: {}", total2);
}

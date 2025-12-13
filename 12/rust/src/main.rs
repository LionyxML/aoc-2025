use anyhow::Result;
use std::fs;

type Shape = (u128, usize);

fn parse(input: &str) -> (Vec<Shape>, Vec<String>) {
    let input = input.replace("\r\n", "\n");
    let chunks: Vec<&str> = input.split("\n\n").collect();

    if chunks.is_empty() {
        return (vec![], vec![]);
    }

    // All chunks except the last are shapes
    let shape_chunks = &chunks[..chunks.len() - 1];
    let region_chunk = chunks[chunks.len() - 1];

    let mut shapes = Vec::new();

    for chunk in shape_chunks {
        let lines: Vec<&str> = chunk.lines().collect();

        // First line is the shape name, rest is the bitmap
        if lines.len() <= 1 {
            continue;
        }

        let flattened: String = lines[1..].join("");
        let area = flattened.chars().filter(|&c| c == '#').count();

        let mut mask: u128 = 0;
        for c in flattened.chars() {
            mask <<= 1;
            if c == '#' {
                mask |= 1;
            }
        }

        shapes.push((mask, area));
    }

    let region_lines = region_chunk
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    (shapes, region_lines)
}

fn is_region_valid(region: &str, shapes: &[Shape]) -> bool {
    let (dims, reqs) = match region.split_once(": ") {
        Some(v) => v,
        None => return false,
    };

    let (w_str, h_str) = match dims.split_once('x') {
        Some(v) => v,
        None => return false,
    };

    let width: usize = match w_str.parse() {
        Ok(v) => v,
        Err(_) => return false,
    };

    let height: usize = match h_str.parse() {
        Ok(v) => v,
        Err(_) => return false,
    };

    let available_area = width * height;

    let required_counts: Vec<usize> = reqs
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap_or(0))
        .collect();

    let mut required_area = 0;

    for (idx, &count) in required_counts.iter().enumerate() {
        if let Some((_, area)) = shapes.get(idx) {
            required_area += count * area;
        }
    }

    available_area >= required_area
}

fn part_1() -> Result<String> {
    let input = fs::read_to_string("data.txt")?;
    let (shapes, region_lines) = parse(&input);

    let count = region_lines
        .iter()
        .filter(|line| is_region_valid(line, &shapes))
        .count();

    Ok(count.to_string())
}

fn main() -> Result<()> {
    let answer = part_1()?;
    println!("Part1: {}", answer);
    Ok(())
}

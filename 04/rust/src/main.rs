use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn file_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    BufReader::new(file).lines().collect()
}

fn get_neighbors_coordenates(line: isize, col: isize) -> Vec<[[isize; 2]; 3]> {
    vec![
        [[line - 1, col - 1], [line - 1, col], [line - 1, col + 1]],
        [[line, col - 1], [line, col], [line, col + 1]],
        [[line + 1, col - 1], [line + 1, col], [line + 1, col + 1]],
    ]
}

fn get_coordenate_neighbors_amount(
    target_coord: [isize; 2],
    vec_lines: &Vec<Vec<String>>,
) -> isize {
    get_neighbors_coordenates(target_coord[0], target_coord[1])
        .iter()
        .flat_map(|line| {
            line.iter().map(|coord| {
                if (target_coord[0] == coord[0] && target_coord[1] == coord[1])
                    || coord[0] < 0
                    || (coord[0] > (vec_lines.len() as isize - 1))
                    || coord[1] < 0
                    || (coord[1] > (vec_lines[0].len() as isize - 1))
                {
                    0
                } else {
                    if vec_lines[coord[0] as usize][coord[1] as usize] == "@" {
                        1
                    } else {
                        0
                    }
                }
            })
        })
        .sum()
}

fn main() {
    let vec_lines: Vec<Vec<String>> = file_lines("data.txt")
        .expect("Could not get lines from file.")
        .iter()
        .map(|l| l.chars().map(|c| c.to_string()).collect())
        .collect();

    // PART 1:

    let fork_limit = 4;

    let mut rows_accessed: isize = 0;

    for (line_idx, line) in vec_lines.iter().enumerate() {
        for (col_idx, _) in line.iter().enumerate() {
            let neighbors =
                get_coordenate_neighbors_amount([line_idx as isize, col_idx as isize], &vec_lines);

            if (&vec_lines[line_idx][col_idx] == "@") && neighbors < fork_limit {
                rows_accessed += 1;
            }
        }
    }

    println!(
        "Part1: {} rows of @ can be accessed by a forklift",
        rows_accessed
    );

    // PART 2:

    let mut vec_lines_2 = vec_lines.clone();
    let mut total_removed = 0;

    loop {
        let mut to_remove = Vec::new();

        for (line_idx, line) in vec_lines_2.iter().enumerate() {
            for (col_idx, _) in line.iter().enumerate() {
                let neighbors = get_coordenate_neighbors_amount(
                    [line_idx as isize, col_idx as isize],
                    &vec_lines_2,
                );

                if vec_lines_2[line_idx][col_idx] == "@" && neighbors < fork_limit {
                    to_remove.push((line_idx, col_idx));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (r, c) in &to_remove {
            vec_lines_2[*r][*c] = "x".to_string();
        }

        total_removed += to_remove.len() as isize;
    }

    println!("Part2: {} total rolls removed", total_removed);
}

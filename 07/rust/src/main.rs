use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    BufReader::new(File::open(path)?).lines().collect()
}

fn main() {
    let file_lines = read_lines("data.txt").expect("Could not read the data file.");

    // Part 1

    let mut grid = file_lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut split_count = 0;

    for idx_row in 0..grid.len() {
        for idx_col in 0..grid[idx_row].len() {
            match grid[idx_row][idx_col] {
                'S' => {
                    if idx_row + 1 < grid.len() {
                        grid[idx_row + 1][idx_col] = '|';
                    }
                }

                '^' => {
                    if idx_row > 0 && grid[idx_row - 1][idx_col] == '|' {
                        split_count += 1;

                        let rows = grid.len();
                        let cols = grid[idx_row].len();

                        if idx_col > 0 {
                            grid[idx_row][idx_col - 1] = '|';
                        }
                        if idx_col + 1 < cols {
                            grid[idx_row][idx_col + 1] = '|';
                        }
                        if idx_row + 1 < rows && idx_col > 0 {
                            grid[idx_row + 1][idx_col - 1] = '|';
                        }
                        if idx_row + 1 < rows && idx_col + 1 < cols {
                            grid[idx_row + 1][idx_col + 1] = '|';
                        }
                    }
                }

                '|' => {
                    if idx_row + 1 < grid.len() {
                        if grid[idx_row + 1][idx_col] == '.' {
                            grid[idx_row + 1][idx_col] = '|';
                        }
                    }
                }

                _ => continue,
            }
        }
    }

    // for row in &grid {
    //     println!("{}", row.iter().collect::<String>());
    // }

    println!("Part1: Total Splits: {}", split_count);

    // Part 2

    let rows = grid.len();
    let cols = grid[0].len();

    let mut timelines = vec![vec![0_usize; cols]; rows];

    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == 'S' {
                if y + 1 < rows {
                    timelines[y + 1][x] = 1;
                }
            }
        }
    }

    for y in 0..rows {
        for x in 0..cols {
            match grid[y][x] {
                '.' => {
                    if y > 0 {
                        let t = timelines[y - 1][x];
                        if t > 0 {
                            timelines[y][x] += t;
                        }
                    }
                }

                '|' => {
                    if y > 0 {
                        let t = timelines[y - 1][x];
                        if t > 0 {
                            timelines[y][x] += t;
                        }
                    }
                }

                '^' => {
                    if y > 0 {
                        let t = timelines[y - 1][x];
                        if t > 0 {
                            if x > 0 {
                                timelines[y][x - 1] += t;
                            }
                            if x + 1 < cols {
                                timelines[y][x + 1] += t;
                            }
                        }
                    }
                }

                _ => {}
            }
        }
    }

    let mut total_timelines = 0usize;
    for x in 0..cols {
        total_timelines += timelines[rows - 1][x];
    }

    println!("Part2: Total timelines = {}", total_timelines);
}

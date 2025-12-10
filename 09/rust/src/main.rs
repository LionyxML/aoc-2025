use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(path: &str) -> Result<Vec<String>, io::Error> {
    BufReader::new(File::open(path)?).lines().collect()
}

fn sort_pair(a: i64, b: i64) -> (i64, i64) {
    if a < b { (a, b) } else { (b, a) }
}

fn abs64(x: i64) -> i64 {
    if x < 0 { -x } else { x }
}

fn main() {
    let file_lines = read_file("data.txt")
        .expect("Can't read file!")
        .iter()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<u128>().expect("Can't convert number!"))
                .collect::<Vec<u128>>()
        })
        .collect::<Vec<Vec<u128>>>();

    // Part 1
    let mut biggest_area = 0u128;

    for pair1 in &file_lines {
        for pair2 in &file_lines {
            let x = pair2[0].abs_diff(pair1[0]) + 1;
            let y = pair2[1].abs_diff(pair1[1]) + 1;

            let area = x * y;

            if area > biggest_area {
                biggest_area = area;
            }
        }
    }

    println!("Part1: {}", biggest_area);

    // Part 2
    let red_points: Vec<(i64, i64)> = file_lines
        .iter()
        .map(|p| (i64::try_from(p[0]).unwrap(), i64::try_from(p[1]).unwrap()))
        .collect();

    #[derive(Clone, Debug)]
    struct Edge {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    }

    let mut edges: Vec<Edge> = Vec::new();

    let (init_x, init_y) = red_points[0];
    let (last_x, last_y) = red_points[red_points.len() - 1];

    for i in 0..red_points.len() - 1 {
        let (fx, fy) = red_points[i];
        let (tx, ty) = red_points[i + 1];
        edges.push(Edge {
            x1: fx,
            y1: fy,
            x2: tx,
            y2: ty,
        });
    }

    edges.push(Edge {
        x1: init_x,
        y1: init_y,
        x2: last_x,
        y2: last_y,
    });

    let intersections = |min_x: i64, min_y: i64, max_x: i64, max_y: i64| {
        for e in &edges {
            let (i_min_x, i_max_x) = sort_pair(e.x1, e.x2);
            let (i_min_y, i_max_y) = sort_pair(e.y1, e.y2);

            if min_x < i_max_x && max_x > i_min_x && min_y < i_max_y && max_y > i_min_y {
                return true;
            }
        }
        false
    };

    let mut biggest_confined_area: i64 = 0;

    let manhattan = |a: (i64, i64), b: (i64, i64)| abs64(a.0 - b.0) + abs64(a.1 - b.1);

    for i in 0..red_points.len() {
        for j in i..red_points.len() {
            let a = red_points[i];
            let b = red_points[j];

            let (min_x, max_x) = sort_pair(a.0, b.0);
            let (min_y, max_y) = sort_pair(a.1, b.1);

            let md = manhattan(a, b);
            if md * md > biggest_confined_area {
                if !intersections(min_x, min_y, max_x, max_y) {
                    let area = (max_x - min_x + 1) * (max_y - min_y + 1);
                    if area > biggest_confined_area {
                        biggest_confined_area = area;
                    }
                }
            }
        }
    }

    println!("Part2: {}", biggest_confined_area);
}

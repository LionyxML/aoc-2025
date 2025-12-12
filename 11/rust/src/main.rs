use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("data.txt").expect("Failed to read data file.");
    let graph = parse_graph(&input);

    // Part 1
    let mut count1 = 0;
    let mut path = Vec::new();
    dfs_part1("you", &graph, &mut path, &mut count1);
    println!("Part 1: Total paths: {}", count1);

    // Part 2
    // Determine which order is possible: fft → dac or dac → fft
    let fft_before_dac = reachable(&graph, "fft", "dac");
    let dac_before_fft = reachable(&graph, "dac", "fft");


    if fft_before_dac && dac_before_fft {
        panic!("Impossible: DAG cannot have both directions unless there's a cycle.");
    }

    let count2 = if fft_before_dac {
        // Order must be: svr → ... → fft → ... → dac → ... → out
        count_paths(&graph, "svr", "fft")
            * count_paths(&graph, "fft", "dac")
            * count_paths(&graph, "dac", "out")
    } else if dac_before_fft {
        // Order must be: svr → ... → dac → ... → fft → ... → out
        count_paths(&graph, "svr", "dac")
            * count_paths(&graph, "dac", "fft")
            * count_paths(&graph, "fft", "out")
    } else {
        // Neither can reach the other → impossible to include both
        0
    };

    println!(
        "Part 2: Total valid paths (must include dac + fft): {}",
        count2
    );
}

fn parse_graph(input: &str) -> HashMap<String, Vec<String>> {
    let mut g = HashMap::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        // format: "aaa: you hhh"
        let mut parts = line.split(':');
        let node = parts.next().unwrap().trim().to_string();
        let rest = parts.next().unwrap_or("").trim();

        let outputs = if rest.is_empty() {
            Vec::new()
        } else {
            rest.split_whitespace().map(|s| s.to_string()).collect()
        };

        g.insert(node, outputs);
    }

    g
}

fn dfs_part1(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    path: &mut Vec<String>,
    count: &mut usize,
) {
    path.push(node.to_string());

    if node == "out" {
        *count += 1;
        path.pop();
        return;
    }

    if let Some(nexts) = graph.get(node) {
        for next in nexts {
            dfs_part1(next, graph, path, count);
        }
    }

    path.pop();
}

fn reachable(graph: &HashMap<String, Vec<String>>, start: &str, target: &str) -> bool {
    let mut stack = vec![start];
    let mut visited = HashSet::new();

    while let Some(node) = stack.pop() {
        if node == target {
            return true;
        }
        if visited.insert(node.to_string()) {
            if let Some(neigh) = graph.get(node) {
                for n in neigh {
                    stack.push(n);
                }
            }
        }
    }

    false
}

// Count paths in a DAG using memoization
fn count_paths(graph: &HashMap<String, Vec<String>>, start: &str, end: &str) -> usize {
    let mut memo = HashMap::new();
    count_paths_rec(start, end, graph, &mut memo)
}

fn count_paths_rec<'a>(
    node: &'a str,
    end: &'a str,
    graph: &'a HashMap<String, Vec<String>>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&cached) = memo.get(node) {
        return cached;
    }

    if node == end {
        return 1;
    }

    let mut total = 0;

    if let Some(nexts) = graph.get(node) {
        for next in nexts {
            total += count_paths_rec(next, end, graph, memo);
        }
    }

    memo.insert(node, total);
    total
}

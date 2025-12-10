use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

// Wee need:

// brew install z3
// export LIBRARY_PATH="/opt/homebrew/lib:$LIBRARY_PATH"
// export DYLD_LIBRARY_PATH="/opt/homebrew/lib:$DYLD_LIBRARY_PATH"
// export Z3_SYS_Z3_LIB_DIR="/opt/homebrew/lib"
// export Z3_SYS_Z3_INCLUDE_DIR="/opt/homebrew/include"
// export Z3_SYS_STATIC=0
// cargo clean
// cargo build

use z3::ast::{Ast, Int};
use z3::{Config, Context, Optimize, SatResult};

fn main() {
    let file = File::open("data.txt").expect("cannot open data.txt");
    let reader = BufReader::new(file);

    let mut total_part1 = 0u64;
    let mut total_part2 = 0u64;

    // Z3 ----
    let mut cfg = Config::new();
    cfg.set_timeout_msec(20000);
    let ctx = Context::new(&cfg);

    for line in reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.trim().is_empty())
    {
        // PARSE THE LEFT SIDE: PATTERN [###..]
        let start_bracket = line.find('[').unwrap();
        let end_bracket = line.find(']').unwrap();
        let pattern = &line[start_bracket + 1..end_bracket];

        let mut target_mask: u64 = 0;
        for (i, c) in pattern.chars().enumerate() {
            if c == '#' {
                target_mask |= 1 << i;
            }
        }

        // PARSE ALL BUTTONS (bitmask for Part 1)
        // BUT ALSO store the *vector list* for Part 2
        let mut buttons_mask = Vec::<u64>::new();
        let mut buttons_vecs = Vec::<Vec<usize>>::new();

        let mut idx = end_bracket;

        while let Some(start) = line[idx..].find('(') {
            let start = idx + start;
            let end = line[start..].find(')').unwrap() + start;
            let inside = &line[start + 1..end];

            let mut mask: u64 = 0;
            let mut vec_btn = Vec::<usize>::new();

            for num in inside.split(',').map(|s| s.trim()) {
                if !num.is_empty() {
                    let k: usize = num.parse().unwrap();
                    mask |= 1 << k;
                    vec_btn.push(k);
                }
            }
            buttons_mask.push(mask);
            buttons_vecs.push(vec_btn);

            idx = end + 1;
        }

        // PARSE TARGET JOLTAGES {a,b,c,...}
        // For Part 2
        let start_brace = line.find('{').unwrap();
        let end_brace = line.find('}').unwrap();
        let inside = &line[start_brace + 1..end_brace];

        let target_jolts: Vec<i64> = inside
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();

        // PART 1
        let goal = target_mask;
        let mut seen = HashSet::<u64>::new();
        let mut q = VecDeque::<(u64, u32)>::new();

        seen.insert(0);
        q.push_back((0, 0));

        let presses = loop {
            let (state, dist) = q.pop_front().unwrap();
            if state == goal {
                break dist;
            }

            for &b in &buttons_mask {
                let next = state ^ b;
                if seen.insert(next) {
                    q.push_back((next, dist + 1));
                }
            }
        };

        total_part1 += presses as u64;

        // PART 2 (Z3 ILP)
        let n_counters = target_jolts.len();
        let n_buttons = buttons_vecs.len();

        let opt = Optimize::new(&ctx);

        // Variables x[i] = number of presses of button i
        let xs: Vec<Int> = (0..n_buttons)
            .map(|i| {
                let v = Int::new_const(&ctx, format!("x_{}", i));
                // x >= 0
                opt.assert(&v.ge(&Int::from_i64(&ctx, 0)));
                v
            })
            .collect();

        // For each counter: sum(x[j] * affect[j]) == target_jolts[i]
        for counter in 0..n_counters {
            // start sum = 0
            let mut sum = Int::from_i64(&ctx, 0);

            for (btn_idx, btn) in buttons_vecs.iter().enumerate() {
                if btn.contains(&counter) {
                    // sum = sum + xs[btn_idx]
                    sum = Int::add(&ctx, &[&sum, &xs[btn_idx]]);
                }
            }
            // assert equality
            opt.assert(&sum._eq(&Int::from_i64(&ctx, target_jolts[counter])));
        }

        // Minimize total presses
        let mut total_presses = Int::from_i64(&ctx, 0);
        for x in &xs {
            total_presses = Int::add(&ctx, &[&total_presses, x]);
        }
        opt.minimize(&total_presses);

        // Solve (check expects a slice of assumptions)
        let result = opt.check(&[]);
        assert!(result == SatResult::Sat);

        let model = opt.get_model().unwrap();
        let presses2: i64 = model.eval(&total_presses, true).unwrap().as_i64().unwrap();

        total_part2 += presses2 as u64;
    }

    println!("Part1: {}", total_part1);
    println!("Part2: {}", total_part2);
}

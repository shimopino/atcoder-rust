use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m]
    }

    let mut pairs = HashSet::new();
    for ai in a {
        ai.windows(2).for_each(|w| {
            let (ai1, ai2) = (w[0], w[1]);
            if ai1 <= ai2 {
                pairs.insert((ai1, ai2));
            } else {
                pairs.insert((ai2, ai1));
            }
        })
    }

    let pairs_len = pairs.len();
    let result = count_combinations(n, 2) - pairs_len;

    println!("{}", result);
}

fn count_combinations(n: usize, r: usize) -> usize {
    if r > n {
        0
    } else {
        (1..=r).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

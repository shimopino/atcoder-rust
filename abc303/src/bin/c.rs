use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        _n: usize,
        m: usize,
        mut h: isize,
        k: isize,
        s: Chars,
        xy: [(isize, isize); m]
    }

    let mut xy_set: HashSet<(isize, isize)> = xy.into_iter().collect();
    let mut point = (0_isize, 0_isize);
    let mut flag = true;

    for c in s {
        point = match c {
            'R' => (point.0 + 1, point.1),
            'L' => (point.0 - 1, point.1),
            'U' => (point.0, point.1 + 1),
            'D' => (point.0, point.1 - 1),
            _ => panic!("unexpected"),
        };

        h -= 1;

        if h < 0 {
            flag = false;
            break;
        }

        if h < k && xy_set.contains(&point) {
            h = k;
            xy_set.remove(&point);
        }
    }

    println!("{}", if flag { "Yes" } else { "No" });
}

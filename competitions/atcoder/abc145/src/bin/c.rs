use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    let mut count = 0;
    let mut dist = 0f64;
    for perms in (0..n).permutations(n) {
        for i in 0..(perms.len() - 1) {
            let (x1, y1) = xy[perms[i]];
            let (x2, y2) = xy[perms[i + 1]];
            dist += ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
        }
        count += 1;
    }

    println!("{}", dist / (count as f64));
}
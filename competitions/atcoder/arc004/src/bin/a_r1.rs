use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i+1..n {
            let dist = (xy[i].0 - xy[j].0).pow(2) + (xy[i].1 - xy[j].1).pow(2);
            ans = max(ans, dist);
        }
    }
    println!("{}", (ans as f64).sqrt());
}
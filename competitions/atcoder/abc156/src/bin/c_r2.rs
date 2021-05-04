use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        x: [i32; n],
    }

    let mut ans = std::i32::MAX;
    for p in 1..101 {
        let mut tmp = 0;
        for xi in &x {
            tmp += (xi - p) * (xi - p);
        }
        ans = min(ans, tmp);
    }
    println!("{}", ans);
}
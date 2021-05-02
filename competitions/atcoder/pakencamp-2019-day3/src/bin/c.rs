use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[u64; m]; n],
    }

    let mut ans: u64 = 0;
    for i in 0..m {
        for j in i+1..m {
            let mut count: u64 = 0;
            for k in 0..n {
                count += max(a[k][i], a[k][j]);
                ans = max(ans, count);
            }
        }
    }
    println!("{}", ans);
}
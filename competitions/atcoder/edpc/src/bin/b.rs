use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i64; n],
    }

    let mut dp: Vec<i64> = vec![std::i64::MAX; n];
    // 初期条件
    dp[0] = 0;

    for i in 1..n {
        for j in 1..k + 1 {
            if i < j {
                continue;
            }
            dp[i] = min(dp[i], dp[i - j] + (h[i] - h[i - j]).abs());
        }
    }
    println!("{}", dp[n - 1]);
}

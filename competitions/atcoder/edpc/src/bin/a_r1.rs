use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let mut dp: Vec<i64> = vec![0; n + 1];
    // 初期条件
    dp[0] = 0;

    for i in 0..n-1 {
        dp[i + 1] = min(dp[i + 1], dp[i] + (h[i] - h[i + 1]).abs());
        dp[i + 2] = min(dp[i + 2], dp[i] + (h[i] - h[i + 2]).abs());
    }

    println!("{}", dp[n - 1]);
}

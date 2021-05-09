use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        abc: [[i64; 3]; n],
    }

    let mut dp = vec![];
    // 初期状態
    // dp.push(abc[0].clone());
    dp.push(vec![abc[0][0], abc[0][1], abc[0][2]]);

    for i in 1..n {
        dp.push(vec![
            abc[i][0] + max(dp[i - 1][1], dp[i - 1][2]),
            abc[i][1] + max(dp[i - 1][2], dp[i - 1][0]),
            abc[i][2] + max(dp[i - 1][0], dp[i - 1][1]),
        ]);
    }
    
    println!("{}", dp[n - 1].iter().max().unwrap());
}

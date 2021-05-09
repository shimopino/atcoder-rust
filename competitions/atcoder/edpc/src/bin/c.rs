use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        abc: [[i64; 3]; n],
    }

    let mut dp: Vec<Vec<i64>> = vec![vec![0; 3]; n + 1];
    for i in 0..n {
        for j in 0..3 {
            for k in 0..3 {
                if j == k {
                    continue;
                }
                dp[i + 1][k] = max(dp[i + 1][k], dp[i][j] + abc[i][k]);
            }
        }
    }
    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}

use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i64; n],
    }

    let mut dp: Vec<i64> = vec![std::i64::MAX; n];
    dp[0] = 0;

    for i in 0..n - 1 {
        for j in 1..k + 1 {
            // 配るDPでは合計が配列長を超えないように注意
            if i + j >= n {
                break;
            }
            dp[i + j] = min(dp[i + j], dp[i] + (h[i] - h[i + j]).abs());
        }
    }
    println!("{}", dp[n - 1]);
}

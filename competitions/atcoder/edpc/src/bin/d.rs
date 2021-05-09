use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [[usize; 2]; n],
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![0; w + 1]; n + 1];

    for i in 0..n {
        for j in 0..w + 1 {
            let weight = wv[i][0];
            let value = wv[i][1];

            if j >= weight {
                // i番目の品物を選ぶとき
                dp[i + 1][j] = max(
                    dp[i][j],
                    dp[i][j - weight] + value
                );
            } else {
                // i番目の品物を選ばないとき
                dp[i + 1][j] = dp[i][j];
            }
        }
    }
    println!("{}", dp[n][w]);
}

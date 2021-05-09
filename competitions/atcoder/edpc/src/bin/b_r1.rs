use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i64; n],
    }

    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;
    for i in 1..n {
        dp[i] = (1..k + 1)
            .filter(|&e| i >= e)
            .map(|e| dp[i - e] + (h[i] - h[i - e]).abs())
            .min()
            .unwrap();
    }
    println!("{}", dp[n - 1]);
}

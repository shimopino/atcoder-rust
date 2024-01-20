use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

// 類似問題
// https://atcoder.jp/contests/abc186/tasks/abc186_c
fn main() {
    input! {
        n: u64,
    }

    let mut n = n - 1;

    if n == 0 {
        println!("0");
        return;
    }

    let mut answer = VecDeque::new();
    while n > 0 {
        answer.push_front(n % 5);
        n /= 5;
    }
    println!("{}", answer.iter().map(|&i| 2 * i).join(""));
}

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut ans = VecDeque::new();
    for (i, ia) in a.iter().enumerate() {
        if i % 2 == 0 {
            ans.push_back(ia);
        }
        if i % 2 == 1 {
            ans.push_front(ia);
        }
    }

    // https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter
    if n % 2 != 0 {
        ans = ans.into_iter().rev().collect();
    }

    for ians in ans {
        print!("{} ", ians);
    }
    println!("");
}
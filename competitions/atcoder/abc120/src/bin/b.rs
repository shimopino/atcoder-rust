use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        a: u32,
        b: u32,
        k: u32,
    }

    let max_value = max(a, b);
    let mut ans = Vec::new();
    for i in 1..=max_value {
        if a % i == 0 && b % i == 0 {
            ans.push(i);
        }
    }
    
    ans.sort();
    ans.reverse();
    println!("{}", ans[(k -1) as usize]);
}
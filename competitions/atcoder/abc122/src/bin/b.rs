use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main() {
    input! {
        mut s: Chars,
    }

    let mut max_count = 0;
    let mut count = 0;
    for i in 0..s.len() {
        if s[i] == 'A' || s[i] == 'C' || s[i] == 'G' || s[i] == 'T' {
            count += 1;
            max_count = max(count, max_count);
        } else {
            count = 0;
        }
    }

    println!("{}", max_count);
}
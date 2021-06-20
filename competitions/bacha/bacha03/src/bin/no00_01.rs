use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut count_0: i64 = 0;
    let mut count_1: i64 = 0;
    for i in 0..s.len() {
        if s[i] == '0' {
            count_0 += 1;
        }
        if s[i] == '1' {
            count_1 += 1;
        }
    }
    println!("{}", 2 * count_1.min(count_0));
}
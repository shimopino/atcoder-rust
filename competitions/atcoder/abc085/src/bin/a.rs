use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }

    s[3] = '8';
    let ans: String = s.iter().collect();
    println!("{}", ans);
}
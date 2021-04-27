use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }

    s.sort();
    s.dedup();

    println!("{}", if s.len() == 2 {"Yes"} else {"No"});
}
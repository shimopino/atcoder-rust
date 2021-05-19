use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    for (i, c) in s.iter().enumerate() {
        print!(
            "{}",
            if i == k - 1 {
                c.to_ascii_lowercase().to_string()
            } else {
                c.to_string()
            }
        );
    }
    println!("");
}

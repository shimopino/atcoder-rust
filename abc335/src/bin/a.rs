use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    for (index, &ic) in s.iter().enumerate() {
        print!("{}", if index == s.len() - 1 { '4' } else { ic });
    }
    println!();
}

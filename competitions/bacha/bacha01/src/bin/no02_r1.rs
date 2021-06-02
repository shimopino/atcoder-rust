use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut start = 0;
    let mut end = 0;
    for (i, c) in s.chars().enumerate() {
        if c == 'A' {
            start = i;
            break;
        }
    }

    for (i, c) in s.chars().rev().enumerate() {
        if c == 'Z' {
            end = s.len() - 1- i;
            break;
        }
    }

    println!("{}", end - start + 1);
}
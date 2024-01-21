use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    s.dedup();

    match &s[..] {
        ['A'] | ['B'] | ['C'] => println!("Yes"),
        ['A', 'B'] | ['B', 'C'] | ['A', 'C'] => println!("Yes"),
        ['A', 'B', 'C'] => println!("Yes"),
        _ => println!("No"),
    }
}

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let mut acc = 0;
    for (i, &ni) in n.iter().rev().enumerate() {
        if ni == '1' {
            acc += 1 << i;
        }
    }

    println!("{}", acc);
}

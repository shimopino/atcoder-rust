use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let answer = n.iter().rev().enumerate().fold(0, |acc, (i, &c)| {
        let signal = c as u8;
        if signal == b'1' {
            return acc + 2_i32.pow(i as u32);
        } else {
            return acc;
        }
    });

    println!("{}", answer);
}

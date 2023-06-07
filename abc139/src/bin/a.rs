use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let count = s
        .into_iter()
        .zip(t.into_iter())
        .filter(|(si, ti)| si == ti)
        .count();

    println!("{}", count);
}

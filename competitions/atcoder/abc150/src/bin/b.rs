use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: u32,
        s: Chars,
    }

    let mut count = 0;
    for w in s.windows(3) {
        if w[0] == 'A' && w[1] == 'B' && w[2] == 'C' {
            count += 1;
        }
    }
    println!("{}", count);
}

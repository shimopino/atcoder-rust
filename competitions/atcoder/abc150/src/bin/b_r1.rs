use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let rs = s.windows(3).filter(|arr| arr == &['A', 'B', 'C']).count();
    println!("{}", rs);
}
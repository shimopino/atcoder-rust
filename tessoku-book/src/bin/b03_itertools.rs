use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let found = a
        .iter()
        .tuple_combinations()
        .any(|(a, b, c)| a + b + c == 1000);

    println!("{}", if found { "Yes" } else { "No" });
}

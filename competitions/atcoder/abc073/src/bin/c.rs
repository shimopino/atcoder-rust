use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut sheets = HashSet::new();
    for ia in a {
        match sheets.contains(&ia) {
            true => sheets.remove(&ia),
            false => sheets.insert(ia),
        };
    }
    println!("{}", sheets.len());
}
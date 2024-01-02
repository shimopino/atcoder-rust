use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64;n],
        q: usize,
        x: [i64; q],
    }

    // 配列を昇順でソートする
    let a_sorted = a.into_iter().sorted().collect::<Vec<_>>();

    for ix in x.iter() {
        match a_sorted.binary_search(ix) {
            Ok(i) => println!("{}", i),
            Err(i) => println!("{}", i),
        }
    }
}

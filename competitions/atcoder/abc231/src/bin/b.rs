use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut hashmap =HashMap::new();

    for is in s.iter() {
        let counter = hashmap.entry(is).or_insert(0);
        *counter += 1;
    }

    let mut vector1: Vec<_> = hashmap.into_iter().collect();
    vector1.sort_by(|x, y| y.1.cmp(&x.1));

    println!("{}", vector1[0].0);
}

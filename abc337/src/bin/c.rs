use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut map = HashMap::new();
    for (i, ia) in a.iter().enumerate() {
        map.insert(ia, i as i64 + 1);
    }

    let mut next = -1;
    for _ in 0..n {
        let answer = map.get(&next).copied().unwrap();
        print!("{} ", answer);
        next = answer;
    }
}

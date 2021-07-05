use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let all_member = k / n;
    let rest = k % n;

    let mut b = a.to_vec();
    b.sort();
    let mut target = HashSet::new();
    for i in 0..rest {
        target.insert(b[i]);
    }

    for i in 0..n {
        if target.contains(&a[i]) {
            println!("{}", all_member + 1);
        } else {
            println!("{}", all_member);
        }
    }
}
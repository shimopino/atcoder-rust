use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: [[isize; d]; n],
    }

    let mut sq = HashSet::new();
    for i in 0..40*10 {
        sq.insert((i as isize).pow(2));
    }

    let mut count = 0;
    for i in 0..n {
        for j in i+1..n {
            let mut d2 = 0;
            for k in 0..d {
                d2 += (x[i][k] - x[j][k]).pow(2);
            }
            if sq.contains(&d2) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
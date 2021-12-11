use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        x: [i64; q]
    }

    a.sort();

    for ix in x.iter() {
        let index = a.lower_bound(ix);
        println!("{}", n - index);
    }

    return;
}

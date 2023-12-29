use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lr: [(usize, usize); q],
    }

    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }

    for &(l, r) in lr.iter() {
        println!("{}", acc[r] - acc[l - 1]);
    }
}

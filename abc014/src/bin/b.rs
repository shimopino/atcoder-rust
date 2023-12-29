use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [i64; n]
    }

    let mut acc = 0;
    for i in (0..n).rev() {
        if (x & 1 << i) != 0 {
            acc += a[i];
        }
    }

    println!("{}", acc);
}

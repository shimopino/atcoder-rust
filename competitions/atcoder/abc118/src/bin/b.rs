use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut count = vec![0; m];
    for _ in 0..n {
        input! {k: usize, aa: [usize; k]}

        for i in 0..k {
            count[aa[i] - 1] += 1;
        }
    }
    println!("{}", count.iter().filter(|&x| *x == n).count());
}
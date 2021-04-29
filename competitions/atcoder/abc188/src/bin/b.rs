use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }

    let mut ans = 0;
    for i in 0..n {
        ans += a[i] * b[i];
    }
    println!("{}", if ans == 0 { "Yes" } else { "No" });
}

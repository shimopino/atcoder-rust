use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [i32; n],
        c: [i32; n],
    }

    let mut ans = 0;
    for i in 0..n {
        if v[i] - c[i] >= 0 {
            ans += v[i] - c[i];
        }
    }
    println!("{}", ans);
}
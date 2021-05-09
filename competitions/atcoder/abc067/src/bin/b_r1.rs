use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut l: [u32; n],
    }

    l.sort();

    let mut ans = 0;
    for i in (n - k..n).rev() {
        ans += l[i];
    }
    println!("{}", ans);
}

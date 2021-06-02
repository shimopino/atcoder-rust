use proconio::input;

fn main() {
    input! {
        n: u128,
    }

    let mut ans: u128 = 0;
    if n >= 1_000 {ans+=n-999}
    if n >= 1_000_000 {ans+=n-999_999}
    if n >= 1_000_000_000 {ans+=n-999_999_999}
    if n >= 1_000_000_000_000 {ans+=n-999_999_999_999}
    if n >= 1_000_000_000_000_000 {ans+=n-999_999_999_999_999}
    println!("{}", ans);
}
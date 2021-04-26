use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }

    let ans = if n * a > b { b } else {n * a};
    println!("{}", ans);
}
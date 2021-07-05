use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let maximum = a * 6;
    let minimum = a * 1;

    println!("{}", if minimum <= b && b <= maximum { "Yes" } else { "No" });
}
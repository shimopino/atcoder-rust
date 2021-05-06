use proconio::input;

fn main() {
    input! {
        n: u32,
        m: u32,
    }

    println!("{}", (n - 1) * (m - 1));
}
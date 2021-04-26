use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }

    println!("{}", (n * a).min(b));
}
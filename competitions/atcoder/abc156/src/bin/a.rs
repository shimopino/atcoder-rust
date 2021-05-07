use proconio::input;

fn main() {
    input! {
        n: u32,
        r: u32,
    }

    if n >= 10 {
        println!("{}", r);
    } else {
        println!("{}", r + 100 * (10 - n));
    }
}
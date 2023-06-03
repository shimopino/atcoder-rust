// use num::integer;
use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    // let result = integer::div_ceil(a, b);
    let result = (a + b - 1) / b;
    println!("{}", result);
}

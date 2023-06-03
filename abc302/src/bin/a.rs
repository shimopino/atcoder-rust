use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    let result = (a + b - 1) / b;
    println!("{}", result);
}

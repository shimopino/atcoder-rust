use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let mut count = 0;
    let mut total = 1;
    while total < b {
        total += a - 1;
        count += 1;
    }
    println!("{}", count);
}
use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let ans = if b % a == 0 { a + b } else { b - a };
    println!("{}", ans);
}

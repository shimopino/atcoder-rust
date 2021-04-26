use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let ans = if a < 10 && b < 10 { a * b } else { -1 };
    println!("{}", ans);
}

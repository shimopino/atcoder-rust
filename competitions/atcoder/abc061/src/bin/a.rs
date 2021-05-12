use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    if c >= a && c <= b {
        println!("Yes");
    } else {
        println!("No");
    }
}
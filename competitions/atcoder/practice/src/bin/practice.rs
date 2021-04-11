use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        s: String,
    };
    let result = a + b + c;
    println!("{} {}", result, s);
}

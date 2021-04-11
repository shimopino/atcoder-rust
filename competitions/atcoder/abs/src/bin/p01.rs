use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    };
    let result = {
        if (a * b) % 2 == 0 {
            "Even"
        } else {
            "Odd"
        }
    };
    println!("{}", result);
}
use proconio::input;

fn main() {
    input! {
        r: u32,
        g: u32,
        b: u32,
    }

    let digit = r * 100 + g * 10 + b;
    println!(
        "{}",
        if digit % 4 == 0 {
            "YES"
        } else {
            "NO"
        }
    );
}
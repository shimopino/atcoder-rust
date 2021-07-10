use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    println!(
        "{}",
        if b >= a {
            b - a + 1
        } else {
            0
        }
    );
}
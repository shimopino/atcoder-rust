use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
    }

    println!(
        "{}",
        if (a..b + 1).any(|x| 100 % x == 0) {
            "Yes"
        } else {
            "No"
        }
    );
}

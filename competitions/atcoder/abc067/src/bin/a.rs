use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    if a % 3 == 0 || b % 3 == 0 || (a + b) % 3 == 0 {
        println!("Possible");
    } else {
        println!("Impossible");
    }
}

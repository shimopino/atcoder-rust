use proconio::input;

fn main() {
    input! {
        k: u32,
        x: u32,
    }

    println!("{}", if 500 * k >= x { "Yes" } else { "No" });
}

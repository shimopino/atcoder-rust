use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{}", if s.starts_with("YAKI") { "Yes" } else { "No" })
}

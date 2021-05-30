use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let ans = if s == "Y" {
        t.to_ascii_uppercase()
    } else {
        t.to_ascii_lowercase()
    };
    println!("{}", ans);
}

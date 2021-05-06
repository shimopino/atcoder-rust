use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("2018{}", &s[4...]);
}
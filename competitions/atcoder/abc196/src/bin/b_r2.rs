use proconio::input;

fn main() {
    input! {
        s: String,
    }
    println!("{}", s.split(".").next().unwrap());
}
use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let answer = (a - b) / 3_f64 + b;

    println!("{}", answer);
}
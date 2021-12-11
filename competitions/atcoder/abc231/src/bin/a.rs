use proconio::input;

fn main() {
    input! {
        d: f64,
    }

    let answer = d / 100_f64;

    println!("{}", answer);
}

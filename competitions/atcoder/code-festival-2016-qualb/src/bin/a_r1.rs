use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = "CODEFESTIVAL2016"
        .chars()
        .zip(s.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count();

    println!("{}", ans);
}
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = 700 + 100 * s.chars().filter(|c| *c == 'o').count();
    println!("{}", ans);
}
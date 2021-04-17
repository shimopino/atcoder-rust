use proconio::input;

fn main() {
    input! {
        n: String,
    }

    let trimed = n.trim_end_matches("0");
    let trimed_rev: String = trimed.chars().rev().collect();

    println!("{}", if trimed == trimed_rev {"Yes"} else {"No"});
}
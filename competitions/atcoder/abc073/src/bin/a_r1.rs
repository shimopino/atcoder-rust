use proconio::input;

fn main() {
    input! {
        n: String,
    }

    let s: Vec<char> = n.chars().collect();
    println!(
        "{}",
        if s[0] == '9' || s[1] == '9' {
            "Yes"
        } else {
            "No"
        }
    );
}

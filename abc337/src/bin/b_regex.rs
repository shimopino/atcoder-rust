use proconio::input;
use regex::Regex;

fn main() {
    input! { s:String }

    let re = Regex::new(r"^A*B*C*$").unwrap();
    println!("{}", if re.is_match(&s) { "Yes" } else { "No" });
}

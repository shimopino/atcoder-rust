use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String,
    }

    // ^     the beginning of text
    // $     the end of text
    // x?    zero or one of x
    let re = Regex::new(r"^A?KIHA?BA?RA?$").unwrap();
    if re.is_match(&s) {
        println!("YES");
    } else {
        println!("NO");
    }
}

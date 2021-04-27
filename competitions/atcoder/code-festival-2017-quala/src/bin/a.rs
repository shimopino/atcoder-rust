use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    println!(
        "{}",
        if (s.len() >= 4) && (s[0] == 'Y' && s[1] == 'A' && s[2] == 'K' && s[3] == 'I') {
            "Yes"
        } else {
            "No"
        }
    );
}

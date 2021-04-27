use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();

    if chars[0] == chars[1] && chars[2] == chars[3] && chars[0] != chars[3] {
        println!("Yes");
    } else {
        println!("No");
    }
}
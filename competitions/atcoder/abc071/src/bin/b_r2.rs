use proconio::input;

fn main() {
    input! {
        s: String,
    }

    for c in b'a'..=b'z' {
        if !s.contains(c as char) {
            println!("{}", c as char);
            return;
        }
    }
    println!("None");
}
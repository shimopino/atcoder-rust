use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut answer = true;
    let mut s_other = s.clone();
    s_other.sort();
    for (i, &is) in s.iter().enumerate() {
        if is == s_other[i] {
            continue;
        } else {
            answer = false;
        }
    }

    println!("{}", if answer { "Yes" } else { "No" });
}

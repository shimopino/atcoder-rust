use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = String::new();
    for c in s {
        if c == '0' { ans.push(c) }
        if c == '1' { ans.push(c) }
        if c == 'B' { ans.pop(); }
    }

    println!("{}", ans);
}
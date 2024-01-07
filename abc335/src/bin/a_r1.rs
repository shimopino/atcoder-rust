use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    }

    s.pop();
    s.push('4');
    println!("{}", s.iter().collect::<String>());
}

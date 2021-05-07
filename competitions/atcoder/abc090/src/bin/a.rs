use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: [Chars; 3],
    }
    println!("{}{}{}", x[0][0], x[1][1], x[2][2]);
}
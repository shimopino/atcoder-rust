use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars
    }

    if (n[0] == n[1] && n[1]== n[2]) || (n[1] == n[2] && n[2]== n[3]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
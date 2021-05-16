use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    }

    if (a + b) == c || (b + c) == a || (c + a) == b {
        println!("Yes");
    } else {
        println!("No");
    }
}
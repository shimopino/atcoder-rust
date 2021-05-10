use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    }

    if (a + b + c) == 17 {
        println!("YES");
    } else {
        println!("NO");
    }
}
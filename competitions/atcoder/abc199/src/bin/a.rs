use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    }

    if (a * a + b * b) < c * c {
        println!("Yes");
    } else {
        println!("No");
    }
}

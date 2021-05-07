use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    }

    if a + b > c + d {
        println!("Left");
    } else if (a + b) == (c + d) {
        println!("Balanced");
    } else {
        println!("Right");
    }
}
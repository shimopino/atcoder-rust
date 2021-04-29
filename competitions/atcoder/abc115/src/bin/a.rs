use proconio::input;

fn main() {
    input! {
        d: u32,
    }

    if d == 22 {
        println!("Christmas Eve Eve Eve");
    } else if d == 23 {
        println!("Christmas Eve Eve");
    } else if d == 24 {
        println!("Christmas Eve");
    } else {
        println!("Christmas");
    }
}
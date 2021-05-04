use proconio::input;

fn main() {
    input! {
        x: u32,
    }

    if x <= 599 {
        println!("8");
    } else if x <= 799 {
        println!("7");
    } else if x <= 999 {
        println!("6");
    } else if x <= 1199 {
        println!("5");
    } else if x <= 1399 {
        println!("4");
    } else if x <= 1599 {
        println!("3");
    } else if x <= 1799 {
        println!("2");
    } else if x <= 1999 {
        println!("1");
    }
}
use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }

    if (x-y).abs() < 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
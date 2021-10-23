use proconio::input;

fn main() {
    input! {
        x: i64,
    }

    if x < 100 {
        println!("No"); 
        return;
    }

    if x % 100 != 0 {
        println!("No");
        return;
    }

    println!("Yes");
}
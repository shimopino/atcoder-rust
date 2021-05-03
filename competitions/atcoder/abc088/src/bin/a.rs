use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
    }

    if (n % 500) <= a {
        println!("Yes");
    } else {
        println!("No");
    }
}
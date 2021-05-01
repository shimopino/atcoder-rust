use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let follower = 2 * a + 100;
    println!("{}", follower - b);
}
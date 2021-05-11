use proconio::input;

fn main() {
    input! {
        x: i32,
        a: i32,
        b: i32,
    }

    let ans = if (x - a).abs() < (x - b).abs() {
        "A"
    } else {
        "B"
    };
    println!("{}", ans);
}

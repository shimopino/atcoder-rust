use proconio::input;

fn main() {
    input! {
        x: u32,
    }

    let ans = if x == 3 || x == 5 || x == 7 {
        "YES"
    } else {
        "NO"
    };
    println!("{}", ans);
}

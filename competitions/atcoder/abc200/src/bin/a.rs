use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let ans = (n + 99) / 100;
    println!("{}", ans);
}

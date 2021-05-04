use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i32; n],
    }

    let ans = (0..101)
        .map(|p| x.iter().fold(0, |acc, xi| acc + (xi - p) * (xi -p)))
        .min()
        .unwrap();

    println!("{}", ans);
}
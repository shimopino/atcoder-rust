use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i32; n],
    }

    let ans = (1..101)
        .map(|p| x.iter().map(|xi| (xi - p) * (xi - p)).sum::<i32>())
        .min()
        .unwrap();

    println!("{}", ans);
}
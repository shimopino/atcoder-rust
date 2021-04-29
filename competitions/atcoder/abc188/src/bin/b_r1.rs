use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }

    let ans = a.iter().zip(b.iter()).map(|(a, b)| a * b).sum::<i32>();
    println!("{}", if ans == 0 { "Yes" } else { "No" });
}

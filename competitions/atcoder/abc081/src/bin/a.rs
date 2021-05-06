use proconio::input;

fn main() {
    input! {
        sss: String,
    }

    let ans = sss.chars()
                 .map(|x| x.to_digit(10).unwrap())
                 .sum::<u32>();
    println!("{}", ans);
}
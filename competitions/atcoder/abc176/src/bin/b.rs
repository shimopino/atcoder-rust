use proconio::input;

fn main() {
    input! {
        n: String,
    }

    let digit_sum = n.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
    println!("{}", if digit_sum % 9 == 0 { "Yes" } else { "No" });
}

use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut min_sum = std::u32::MAX;
    for i in 1..n {
        let mut a = i;
        let mut b = n - i;

        let mut a_sum = 0;
        while a > 0 {
            a_sum += a % 10;
            a /= 10;
        }

        let mut b_sum = 0;
        while b > 0 {
            b_sum += b % 10;
            b /= 10;
        }

        min_sum = std::cmp::min(min_sum, a_sum + b_sum);
    }
    println!("{}", min_sum);
}
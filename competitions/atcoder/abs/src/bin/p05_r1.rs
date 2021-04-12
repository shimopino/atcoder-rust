use proconio::input;

fn sum_digits(x: u32) -> u32 {
    let mut x = x;
    let mut sum = 0;
    while x > 0 {
        sum += x % 10;
        x /= 10;
    }
    sum
}

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    };

    let mut sum = 0;
    for i in 1..n+1 {
        let s = sum_digits(i);
        if a <= s && s <= b {
            sum += i;
        }
    }
    println!("{}", sum);
}
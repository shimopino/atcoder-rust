use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }

    let mut ans = 0;
    for i in 1..n + 1 {
        let mut x = i;
        let mut sum = 0;
        while x > 0 {
            sum += x % 10;
            x /= 10;
        }
        if a <= sum && sum <= b {
            ans += i;
        }
    }
    println!("{}", ans);
}

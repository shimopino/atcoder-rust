use proconio::input;

fn main() {
    input! {
        mut p: usize,
    }

    let mut coin = 10;
    let mut ans = 0;
    while p > 0 {
        let amount = p / factorial(coin);
        ans += amount;
        p -= amount * factorial(coin);
        coin -= 1;
    }

    println!("{}", ans);
}

fn factorial(n: usize) -> usize {
    if n == 1 {
        return 1;
    }

    return n * factorial(n - 1);
}

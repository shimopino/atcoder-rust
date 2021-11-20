use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut answer = 0;
    for a in (1..).take_while(|i| i *i *i <= n) {
        for b in (a..).take_while(|i| a * i * i <= n) {
            answer += n / (a * b) - b + 1;
        }
    }

    println!("{}", answer);
}

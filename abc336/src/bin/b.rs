use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // n を2進数表記した時の末尾の0の数を数える
    let mut count = 0;
    let mut m = n;
    while m % 2 == 0 {
        count += 1;
        m /= 2;
    }
    println!("{}", count);
}

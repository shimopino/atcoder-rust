use proconio::input;

fn main() {
    input! {
        s: i64,
        t: i64
    }

    let mut count = 0;
    for a in 0..=1000 {
        for b in 0..=(s - a) {
            for c in 0..1001 {
                if a + b + c <= s && a * b * c <= t {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

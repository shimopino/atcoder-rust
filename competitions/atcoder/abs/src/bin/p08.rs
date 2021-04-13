use proconio::input;

fn main() {
    input! {
        n: i32,
        y: i32,
    };

    let mut a: i32 = -1;
    let mut b: i32 = -1;
    let mut c: i32 = -1;

    for i in 0..=n {
        for j in 0..=n - i {
            let k = n - i - j;
            let total = 10000 * i + 5000 * j + 1000 * k;
            if total == y {
                a = i;
                b = j;
                c = k;
                break;
            }
        }
    }

    println!("{} {} {}", a, b, c);
}
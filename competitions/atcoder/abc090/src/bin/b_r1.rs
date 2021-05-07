use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    let mut count = 0;
    for i in a..b + 1 {
        let s = i % 10;         // 1桁目
        let t = i / 10000 % 10; // 5桁目
        let u = i / 10 % 10;    // 2桁目
        let v = i / 1000 % 10;  // 3桁目
        if (s == t) && (u == v) {
            count += 1;
        }
    }
    println!("{}", count);
}

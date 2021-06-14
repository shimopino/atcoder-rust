fn main() {
    proconio::input! {
        a: i128,
        b: i128,
        c: i128,
    }

    // 累乗が奇数か偶数かで結果は変わる
    if c % 2 == 0 {
        // 偶数
        if a.abs() > b.abs() {
            println!(">");
            return;
        } else if a.abs() < b.abs() {
            println!("<");
            return;
        } else {
            println!("=");
            return;
        }
    } else {
        // 奇数
        if a > b {
            println!(">");
            return;
        } else if a < b {
            println!("<");
            return;
        } else {
            println!("=");
            return;
        }
    }
}
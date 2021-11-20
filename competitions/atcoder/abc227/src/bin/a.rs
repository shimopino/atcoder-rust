use proconio::input;

/// 考慮ポイント
/// もしも全体の人数が無限大の場合はどうなるのか考える
/// 後は、計算が合うようにあまりを求めればいい
fn main() {
    input! {
        n: i64,
        k: i64,
        a: i64,
    }

    let answer = (a + k - 1) % n;

    println!("{}", if answer == 0 { n } else { answer });
}

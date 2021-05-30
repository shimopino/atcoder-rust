use proconio::input;

fn main() {
    input! {
        s: String,
    }

    // '0'はASCIIコードで48、'9'はASCIIコードで57
    // '0'を引くことで9の場合は 57 - 48 = 9 と計算できる
    let ans = s.chars().map(|c| c as u64 - '0' as u64).sum::<u64>();
    println!("{}", if ans % 9 == 0 { "Yes" } else { "No" });
}

use proconio::*;
 
fn main() {
    input! { n: String }
 
    let mut ans = None;

    // 桁数分のbit全探索を実行する
    for bit in 1u64..(1 << n.len()) {
        // enumerate: 文字の要素ごとに桁位置とその要素の値を抽出する
        // fold: 抽出した桁位置の合計を加算していく
        let digit = n.chars().enumerate().fold(0, |acc, (i, c)| {
            // i桁目が 1 になっている場合に抽出する
            acc + if (bit & (1 << i)) != 0 {
                c.to_digit(10).unwrap()
            } else {
                0
            }
        });
 
        if digit % 3 == 0 {
            ans = Some(std::cmp::min(
                ans.unwrap_or(n.len() as i32),
                (n.len() as u32 - bit.count_ones()) as i32
            ));
        }
    }
 
    // ansがNoneのまま、つまり3の倍数で割れるパターンがないときは-1
    println!(
        "{}",
        ans.unwrap_or(-1)
    );
}
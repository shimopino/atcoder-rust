use proconio::input;

fn main() {
    input! {
        n: i64,
        l: i64,
        k: i64,
        a: [i64; n]
    }

    // 2分探索
    let mut left: i64 = -1;
    let mut right: i64 = l + 1;
    // left と right の位置関係が保たれている間
    // => left と right が重なったり、逆転したりすると抜ける
    while right - left > 1 {
        let mid = (left + right) / 2;
        if check(mid, l, k, &a) {
            // スコアを mid 以上にすることが可能な場合
            left = mid;
        } else {
            // 不可能な場合
            right = mid;
        }
    }
    println!("{}", left);
}

fn check(
    mid: i64,
    l: i64,
    k: i64,
    a: &Vec<i64>,
) -> bool {

    let mut num = 0; // 最終的に切断して数
    let mut pre = 0; // 1つ前の切れ目の位置

    for ia in a.into_iter() {
        if ia - pre >= mid {
            // mid 以上になる場合は切断して数え上げる
            num += 1;
            pre = *ia;
        }
    }

    // 最後の1ピースが mid 以上になるのか計算する
    if l - pre >= mid {
        num += 1;
    }

    return num >= k + 1;
}
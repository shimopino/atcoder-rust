use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m]
    }

    let mut pairs = HashSet::new();
    for a in a {
        // ベクタから指定した要素数分取り出してループで回していく
        for a in a.windows(2) {
            let (ai1, ai2) = (a[0] - 1, a[1] - 1);
            // 大小比較を行う場合は、min/max関数を利用すればいい
            pairs.insert((ai1.min(ai2), ai1.max(ai2)));
        }
    }

    // 全ての組み合わせの数から仲の良い組み合わせの数を減らす
    println!("{}", (n * (n - 1) / 2) - pairs.len());
}

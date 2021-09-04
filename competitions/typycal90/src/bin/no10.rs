use proconio::input;

/**
 * 前提
 * - 全探索をぐっちょくに実施した場合、生徒全員でN通り、
 *   質問の数でQ通り、探索範囲は最悪ケースでN通りとなる
 * - つまり合計の計算量は O(N * (NQ))　となり、計算
 *   オーダーは 10 ^ 15 になってしまい計算がTLEとなる。
 * 
 * 戦略
 * - 累積和を使用する
 * - それぞれの組に応じて、i番目の学生番号と期末試験点数
 *   の合計点を事前に計算しておく。
 * - 質問に対しては、それぞれの組の累積和に対して、学籍
 *   番号でクエリを投げ、該当する期末点数の差を取ればいい
 * - 計算オーダー
 *   - 事前計算: O(N)
 *   - クエリ: O(Q)
 *   - 合計: O(N + Q)
 */
fn main() {
    input! {
        n: usize,
        cp: [(i64, i64); n],
        q: usize,
        lr: [(i64, i64); q],
    }

    let mut acm_a = vec![0_i64; n + 1];
    let mut acm_b = vec![0_i64; n + 1];

    for (i, (ic,ip)) in cp.iter().enumerate() {
        acm_a[i + 1] += acm_a[i];
        acm_b[i + 1] += acm_b[i];

        if *ic == 1 {
            acm_a[i + 1] += *ip;
        } else {
            acm_b[i + 1] += *ip;
        }
    }

    // println!("{:?}", acm_a);
    // println!("{:?}", acm_b);

    for (_i, (l, r)) in lr.into_iter().enumerate() {
        let right = r as usize;
        let left = l - 1;
        let total_a = acm_a[right] - acm_a[left as usize];
        let total_b = acm_b[right] - acm_b[left as usize];

        print!("{} {}", total_a, total_b);
        println!("")
    }

    return;
}
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; k]; n],
    }

    println!(
        "{}",
        if dfs(0, 0, n, k, &t) {
            "Found"
        } else {
            "Nothing"
        }
    );
}

/// Args
///   * num_q: 今の質問数 (階層数に該当する)
///   * value: 質問の値
///   * n: 全質問数
///   * k: 選択肢の数
///   * t: 質問と整数値の組み合わせ
fn dfs(
    num_q: usize,
    value: usize,
    n: usize,
    k: usize,
    t: &Vec<Vec<usize>>
) -> bool {
    if num_q == n {
        // 全階層の質問を探索した後、0になっているか確認する
        return value == 0;
    }

    for i in 0..k {
        // 探索して value が0になる組み合わせがあれば true
        if dfs(num_q + 1, value ^ t[num_q][i], n, k, t) {
            return true;
        }
    }

    // 探索して value が0になる組み合わせがない場合は false
    return false;
}
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // 0桁の状態から順番に計算していく
    let count = dfs(0, n);
    println!("{}", count);
}

fn dfs(
    value: usize,
    n: usize
) -> usize {

    // 探索対象の値が入力値よりも大きい場合は探索終了
    if value > n {
        return 0;
    }

    let mut count = if "753".chars().all(|c| {
        value.to_string().contains(c)
    }) {
        1
    } else {
        0
    };

    for c in vec![3, 5, 7] {
        let next_value = value * 10 + c;
        count += dfs(next_value, n);
    }

    return count;
}
use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut all: Vec<i64> = vec![];
    for v in 1..10 {
        dfs(1, v, &mut all);
    }

    all.sort();

    println!("{}", all[k - 1]);
}

fn dfs(
    digit: i64,
    value: i64,
    all: &mut Vec<i64>
) {
    // 検索対象の場合は追加していく
    all.push(value);

    // 10桁の場合は後続の探索を実行せずに打ち切る
    if digit == 10 {
        return;
    }

    // 次の桁を探索する
    for j in -1..=1 {
        let addition = (value % 10) + j;
        if addition >= 0 && addition <= 9 {
            dfs(digit + 1, value * 10 + addition, all);
        }
    }
}
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let s = String::new();
    dfs(s, b'a', n);
}

fn dfs(
    s: String,
    max_char: u8,
    n: usize
) {

    // 探索対象の値が入力値よりも大きい場合は探索終了
    if s.len() == n {
        println!("{}", s);
        return;
    }

    for next_c in b'a'..=max_char {
        let mut next_s = s.clone();
        next_s.push(next_c as char);
        if next_c < max_char {
            // 次の探索範囲が最大より小さい場合は同じ最大値で探索
            dfs(next_s, max_char, n);
        } else {
            // 次の探索値 == 最大値のときに次の文字を探索する
            dfs(next_s, max_char + 1, n);
        }
    }
}
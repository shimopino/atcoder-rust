use proconio::input;

fn main() {
    input! {
        n: usize
    }

    dfs("".to_string(), n);
}

fn dfs(s: String, n: usize) {
    // パスワードの文字列が指定の長さの場合になった場合はカウント
    if s.len() == n {
        println!("{}", s);
        return;
    }

    for c in vec!["a", "b", "c"] {
        dfs(format!("{}{}", s, c), n);
    }
}
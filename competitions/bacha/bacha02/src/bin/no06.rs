use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        sc: [(usize, char); m],
    }

    let mut ans = -1;
    for target in 0..1000 {
        let digit: Vec<char> = target.to_string().chars().collect();

        // 指定された桁数のときのみ計算を行う
        if digit.len() == n {
            if sc.iter().all(|(s, c)| matches!(digit.get(s-1), Some(x) if x == c)) {
                ans = target;
                break;
            }
        }
    }
    println!("{}", ans);
}
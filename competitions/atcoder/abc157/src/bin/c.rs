use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        pair: [(usize, char); m],
    }

    let mut ans = -1;
    for target in 0..10i32.pow(n) {
        let digit: Vec<char> = target.to_string().chars().collect();

        if digit.len() == n {
            if pair.iter().all(|(s, c| matches!(digit.get(*s-1), Some(x) if *x == *c))) {
                ans = target;
                break;
            }
        }
    }
    println!("{}", ans);
}
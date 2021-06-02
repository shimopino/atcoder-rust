use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 1000;

    for vec in s.windows(3) {
        let num = vec.iter()
                     .collect::<String>()
                     .parse::<i32>()
                     .unwrap();
        ans = ans.min((num - 753).abs());
    }
    
    println!("{}", ans);
}
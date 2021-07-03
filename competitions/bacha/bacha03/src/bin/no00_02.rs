use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = String::new();
    for c in s {
        match c {
            '0' => ans.push('0'),
            '1' => ans.push('1'),
            'B' => {
                if ans.len() > 0 {
                    ans.pop();
                }
            },
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}
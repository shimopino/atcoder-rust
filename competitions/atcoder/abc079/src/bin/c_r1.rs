use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let abcd: Vec<i32> = s.iter().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let a = abcd[0];
    let bb = vec![abcd[1] , -abcd[1]];
    let cc = vec![abcd[2] , -abcd[2]];
    let dd = vec![abcd[3] , -abcd[3]];

    for b in bb {
        for c in cc {
            for d in dd {
                if a + b + c + d == 7 {
                    println!("{}{:+}{:+}{:+}=7", a, b, c, d);
                    return;
                }
            }
        }
    }
}
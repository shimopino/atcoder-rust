use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let a = s[0].to_digit(10).unwrap() as i32;
    let b = s[1].to_digit(10).unwrap() as i32;
    let c = s[2].to_digit(10).unwrap() as i32;
    let d = s[3].to_digit(10).unwrap() as i32;

    if a + b + c + d == 7 { println!("{}+{}+{}+{}=7", a, b, c, d) } 
    else if a +  b + c - d == 7 { println!("{}+{}+{}-{}=7", a, b, c, d) }
    else if a +  b - c + d == 7 { println!("{}+{}-{}+{}=7", a, b, c, d) }
    else if a -  b + c + d == 7 { println!("{}-{}+{}+{}=7", a, b, c, d) }
    else if a +  b - c - d == 7 { println!("{}+{}-{}-{}=7", a, b, c, d) }
    else if a -  b - c + d == 7 { println!("{}-{}-{}+{}=7", a, b, c, d) }
    else if a -  b + c - d == 7 { println!("{}-{}+{}-{}=7", a, b, c, d) }
    else { println!("{}-{}-{}-{}=7", a, b, c, d) }
}
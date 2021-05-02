use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let len = s.len(); 
 
    let mut total: u64 = 0;
    for bit in 0..(1 << len) {
        let mut ans = String::new();
        for i in 0..len {
            ans.push_str(&s[i].to_string());
            if bit & (1 << len) != 0b1 {
                ans.push_str("+");
            }
        }

        println!("{}", ans);
        for j in ans.split("+") {
            println!("{}", j);
        }

        total += ans.split("+")
                    .map(|x| x.parse::<u64>().unwrap())
                    .fold(0, |acc, val| acc + val);
    }
    println!("{}", total);
}
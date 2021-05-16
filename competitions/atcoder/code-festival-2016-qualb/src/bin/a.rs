use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut count = 0;
    let correct = "CODEFESTIVAL2016";
    for (s1, s2) in s.chars().zip(correct.chars()) {
        if s1 != s2 {
            count += 1;
        }
    }

    println!("{}", count);
}

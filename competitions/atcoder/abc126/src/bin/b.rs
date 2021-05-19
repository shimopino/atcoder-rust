use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let (a, b) = (s[..2].parse::<i32>().unwrap(),
                  s[2..].parse::<i32>().unwrap());
    
    // https://stackoverflow.com/questions/52932572/what-is-the-dot-dot-equals-operator-in-rust
    // start..=end as a pattern in a match expression
    let ans = match (a, b) {
        (1..=12, 1..=12) => "AMBIGUOUS",
        (1..=12, _) => "MMYY",
        (_, 1..=12) => "YYMM",
        _ => "NA",
    };

    println!("{}", ans);
}
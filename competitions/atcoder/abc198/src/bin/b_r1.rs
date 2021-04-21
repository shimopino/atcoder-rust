use proconio::input;

fn main() {
    input! {
        n: String,
    }

    let mut flag = false;
    for i in 0..10 {
        let t = format!("{}{}", "0".repeat(i), n);
        let r = t.chars().rev().collect::<String>();
        if t == r {
            flag = true;
            break;
        }
    }
    println!("{}", if flag { "Yes" } else { "No" });
}

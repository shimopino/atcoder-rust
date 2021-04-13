use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let mut t = String::new();
    for c in s.chars().rev() {
        t.push(c);
        if t == "maerd" || t == "remaerd" || t == "esare" || t == "resare" {
            t = "".to_string();
        }
    }
    let ans = if t == "" { "YES" } else { "No" };
    println!("{}", ans);
}

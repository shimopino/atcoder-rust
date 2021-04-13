use proconio::input;

fn main() {
    input! {
        mut s: String,
    };
    let a = ["dream", "dreamer", "erase", "eraser"];
    loop {
        let mut b = 0;
        for t in &a {
            if s.ends_with(t) {
                s.truncate(s.len() - t.len());
                b = 1;
                break;
            }
        }
        if b == 0 { break; }
    }
    println!("{}", ["YES", "NO"][(s.len() > 0) as usize]);
}
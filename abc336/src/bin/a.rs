use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let s = format!("L{}ng", "o".repeat(n));
    println!("{}", s);
}

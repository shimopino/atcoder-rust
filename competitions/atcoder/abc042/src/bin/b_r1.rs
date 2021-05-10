use proconio::input;

fn main() {
    input! {
        n: u32,
        _l: u32,
        mut s: [String; n],
    }

    s.sort();
    println!("{}", s.join(""));
}
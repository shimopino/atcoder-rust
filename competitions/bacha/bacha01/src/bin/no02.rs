use proconio::input;

fn main() {
    input! {
        s: String,
    }

    // leftmost
    let start = s.find('A').unwrap();
    // rightmost
    let end = s.rfind('Z').unwrap();
    println!("{}", end - start + 1);
}
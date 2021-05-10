use proconio::input;

fn main() {
    input! {
        n: u32,
        _l: u32,
        mut s: [String; n],
    }

    s.sort_by(|a, b| a.cmp(b));
    for is in s.iter() {
        print!("{}", is);
    }
    println!("");
}
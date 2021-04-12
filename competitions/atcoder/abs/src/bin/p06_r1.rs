use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; n],
    };

    a.sort();

    let mut result = 0;
    for (idx, i) in a.iter().rev().enumerate() {
        if idx % 2 == 0 {
            result += i;
        } else {
            result -= i;
        }
    }
    println!("{}", result);
}
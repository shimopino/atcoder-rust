use proconio::input;

fn main() {
    input! {
        n: u32,
        mut d: [u32; n],
    };

    d.sort();
    d.dedup();
    println!("{}", d.len());
}
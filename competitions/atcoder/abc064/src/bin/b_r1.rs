use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let min = a.iter().min().unwrap();
    let max = a.iter().max().unwrap();

    println!("{}", max - min);
}
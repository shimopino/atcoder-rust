use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(u32, u32); n],
    }

    let mut sum = 0;
    for (l, r) in lr {
        sum += r - l + 1;
    }
    println!("{}", sum);
}
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }

    a.sort();
    let mut distance = 0;
    for w in a.windows(2) {
        distance += w[1] - w[0]
    }
    println!("{}", distance);
}



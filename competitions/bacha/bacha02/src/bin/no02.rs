use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i32,
        m: [i32; n],
    }

    let mut total = 0;
    let mut minimum = std::i32::MAX;
    for i in 0..n {
        total += m[i];
        minimum = minimum.min(m[i]);
    }

    let ans = (m.len() as i32) + (x - total) / minimum;
    println!("{}", ans);
}
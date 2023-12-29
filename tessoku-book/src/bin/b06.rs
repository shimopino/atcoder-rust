use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + if a[i] == 1 { 1 } else { -1 }
    }

    for &(l, r) in lr.iter() {
        println!(
            "{}",
            match (acc[r] - acc[l - 1]).cmp(&0) {
                std::cmp::Ordering::Greater => "win",
                std::cmp::Ordering::Equal => "draw",
                std::cmp::Ordering::Less => "lose",
            }
        )
    }
}

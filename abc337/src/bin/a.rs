use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let (x_sum, y_sum) = xy
        .iter()
        .fold((0, 0), |acc, &(x, y)| (acc.0 + x, acc.1 + y));

    println!(
        "{}",
        match x_sum.cmp(&y_sum) {
            std::cmp::Ordering::Greater => "Takahashi",
            std::cmp::Ordering::Equal => "Draw",
            std::cmp::Ordering::Less => "Aoki",
        }
    )
}

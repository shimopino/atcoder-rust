use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u32,
        ct: [(u32, u32); n]
    }

    let ct_filtered = ct.iter()
                        .filter(|ict| ict.1 <= t);
    let mut min_cost = std::u32::MAX;
    for ict in ct_filtered {
        min_cost = std::cmp::min(ict.0, min_cost);
    }
    if min_cost == std::u32::MAX {
        println!("TLE");
    } else {
        println!("{}", min_cost);
    }
}
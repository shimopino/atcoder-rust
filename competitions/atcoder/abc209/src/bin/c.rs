use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c: [u128; n],
    }

    c.sort();
    let mut total: u128 = 1;
    for (i, ic) in c.into_iter().enumerate() {
        total *= ic - i as u128;
        total %= 1_000_000_000 + 7;
    }

    println!("{}", total);
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut count = 1;
    let mut total = 0;
    for ia in a {
        if count % 2 == 0 {
            total += ia - 1;
        } else {
            total += ia;
        }
        count += 1;
    }

    println!(
        "{}",
        if total <= x {
            "Yes"
        } else {
            "No"
        }
    )
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut is_known = vec![false; n];
    let mut current = x - 1;

    while is_known[current] == false {
        is_known[current] = true;
        current = a[current] - 1;
    }

    let total = is_known.into_iter().filter(|x| *x == true).count();
    println!("{}", total);
}

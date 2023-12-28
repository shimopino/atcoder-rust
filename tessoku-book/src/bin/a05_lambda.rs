use proconio::input;

fn main() {
    input! {
      n: i32,
      k: i32,
    }

    let count = (1..n + 1)
        .flat_map(|x| (1..n + 1).map(move |y| x + y))
        .filter(|&xy| (1..n + 1).contains(&(k - xy)))
        .count();

    println!("{}", count);
}

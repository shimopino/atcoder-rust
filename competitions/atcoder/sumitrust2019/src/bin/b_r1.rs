use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let ans = (1..50001).find(|x| x * 108 / 100 == n);
    match ans {
        Some(i) => println!("{}", i),
        None => println!(":(")
    }
}
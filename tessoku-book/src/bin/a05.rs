use proconio::input;

fn main() {
    input! {
      n: i32,
      k: i32,
    }

    let mut count = 0;
    for i in 1..=n {
        for j in 1..=n {
            let nokori = k - (i + j);
            if 1 <= nokori && nokori <= n {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

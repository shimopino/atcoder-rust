use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let answer = (2..=1000)
                .max_by_key(|&i| a.iter().filter(|&a| *a % i == 0).count())
                .unwrap();
    println!("{}", answer);
}
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let answer = (2..1001)
                .map(|k| (a.iter().filter(|&v| v % k == 0).count(), k))
                .max()
                .unwrap().1;
    println!("{}", answer);
}
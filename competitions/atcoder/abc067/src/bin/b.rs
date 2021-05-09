use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut l: [u32; n],
    }

    l.sort();
    l.reverse();

    let ans: u32 = l.iter().take(k).sum();
    println!("{}", ans);
}

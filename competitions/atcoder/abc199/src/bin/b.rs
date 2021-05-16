use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }

    let a_max = a.iter().max().unwrap();
    let b_min = b.iter().min().unwrap();
    let ans = b_min - a_max + 1;
    println!("{}", ans.max(0));
}

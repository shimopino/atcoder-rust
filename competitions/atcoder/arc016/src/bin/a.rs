use proconio::input;

fn main() {
    input! {
        n: u32,
        m: u32,
    }

    let mut ans = 0;
    for i in 1..n+1 {
        if i != m {
            ans = i;
        }
    }
    println!("{}", ans);
}
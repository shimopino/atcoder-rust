use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, usize); n],
        x: String,
    }

    let mut flag = false;
    let mut ans = 0;
    for i in 0..n {
        if flag {
            ans += st[i].1;
        }
        if st[i].0 == x {
            flag = true;
        }
    }
    println!("{}", ans);
}

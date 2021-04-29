use proconio::input;

fn main() {
    input! {
        a: usize,
        st: [(String, usize); a],
        x: String,
    }

    let mut ans = 0;
    for i in (0..a).rev() {
        if st[i].0 == x {
            break;
        }
        ans += st[i].1;
    }
    println!("{}", ans);
}
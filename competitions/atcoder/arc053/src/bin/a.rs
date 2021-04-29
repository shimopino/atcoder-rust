use proconio::input;

fn main() {
    input! {
        h: u32,
        w: u32,
    }

    let mut ans = 0;
    for _i in 0..h {
        ans += w - 1;
    }
    for _i in 0..w {
        ans += h - 1;
    }
    println!("{}", ans);
}
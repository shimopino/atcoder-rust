use proconio::input;

fn main() {
    input! {
        r: i64,
        x: i64,
        y: i64,
    }

    let dist = x * x + y * y;
    let mut ans = 1;
    loop {
        if r * r * ans * ans >= dist {
            break;
        }
        ans += 1;
    }
    if ans == 1 {
        if r * r != dist {
            ans = 2;
        }
    }
    println!("{}", ans);
}

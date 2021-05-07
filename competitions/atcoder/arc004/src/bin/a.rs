use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n]
    }

    let mut ans = 0f32;
    for i in 0..n {
        for j in i+1..n {
            let xx = (xy[i].0 - xy[j].0) * (xy[i].0 - xy[j].0);
            let yy = (xy[i].1 - xy[j].1) * (xy[i].1 - xy[j].1);
            let dist = ((xx + yy) as f32).sqrt();

            if dist > ans {
                ans = dist;
            }
        }
    }
    println!("{}", ans);
}
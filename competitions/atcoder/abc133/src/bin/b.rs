use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: [[i32; d]; n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i+1..n {
            let mut distance  = 0;
            for k in 0..d {
                distance += (x[i][k] - x[j][k]) * (x[i][k] - x[j][k]);
            }
            for k in 1..distance+1 {
                if k * k == distance {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
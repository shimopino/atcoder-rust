use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n],
    }

    let mut total = 0_f64;
    for (a, b) in ab.iter() {
        total = total + a / b;
    }

    let mut ans = 0_f64;
    let mut half = total / 2_f64;
    for (a, b) in ab.iter() {
        let necessary_time = a / b;

        // 残り時間が、今の導火線を通過するのに必要な時間に達していない時
        if half <= necessary_time {
            ans += half * b;
            break;
        }

        // 残り時間が、今の導火線を通過するのに必要な時間より大きい場合
        if half > necessary_time {
            ans += a;
            half -= a / b;
        }
    }

    println!("{}", ans);
}
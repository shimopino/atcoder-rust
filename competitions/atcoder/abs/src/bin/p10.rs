use proconio::input;

fn abs(x: u32, y: u32) -> u32 {
    x.max(y) - x.min(y)
}

fn main() {
    input! {
        n: u32,
        points: [(u32, u32, u32); n],
    }

    let mut prev_t = 0;
    let mut prev_x = 0;
    let mut prev_y = 0;
    for (t, x, y) in points.iter() {
        let diff_t = t - prev_t;
        let diff_xy = abs(*x, prev_x) + abs(*y, prev_y);
        if (diff_xy > diff_t) || (diff_t % 2 == diff_xy % 2) {
            println!("No");
            return;
        }

        prev_t = *t;
        prev_x = *x;
        prev_y = *y;
    }
    println!("Yes");
}
use proconio::input;

fn main() {
    input! {
        n: i32,
        mut points: [(i32, i32, i32); n],
    }

    points.insert(0, (0, 0, 0));
    let yes = points.windows(2).all(|w| {
        let (t, x, y) = w[0];
        let (nt, nx, ny) = w[1];
        let time = nt - t;
        let dist = (nx - x).abs() + (ny - y).abs();
        dist <= time && time % 2 == dist % 2
    });

    println!("{}", if yes { "Yes" } else { "No" });
}

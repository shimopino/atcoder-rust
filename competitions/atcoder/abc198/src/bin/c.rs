use proconio::input;

fn main() {
    input! {
        r: f64,
        x: f64,
        y: f64,
    }

    let dist = (x * x + y * y).sqrt();

    if dist < r {
        println!("2");
    } else {
        println!("{}", (dist / r).ceil() as usize);
    }
}

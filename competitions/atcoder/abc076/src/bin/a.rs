use proconio::input;

fn main() {
    input! {
        r: f32,
        g: f32,
    }

    println!("{}", 2f32 * g - r);
}
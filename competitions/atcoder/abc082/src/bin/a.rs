use proconio::input;

fn main() {
    input! {
        a: f32,
        b: f32,
    }

    let mean = (a + b) / 2f32;
    println!("{}", mean.ceil());
}
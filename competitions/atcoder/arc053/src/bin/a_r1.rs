use proconio::input;

fn main() {
    input! {
        h: u32,
        w: u32,
    }

    println!("{}", h * (w - 1) + w * (h - 1));
}

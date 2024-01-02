use proconio::input;

fn main() {
    input! {
        b: i32,
        g: i32
    }

    println!("{}", if b > g { "Bat" } else { "Glove" });
}

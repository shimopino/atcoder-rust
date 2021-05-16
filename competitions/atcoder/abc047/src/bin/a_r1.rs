use proconio::input;

fn main() {
    input! {
        mut x: [u32; 3],
    }

    x.sort();
    println!("{}", if x[0] + x[1] == x[2] { "Yes" } else { "No" });
}

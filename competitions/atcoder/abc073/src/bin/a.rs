use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let syo = n / 10;
    let amari = n % 10;
    println!("{}", if syo == 9 || amari == 9 { "Yes" } else { "No" });
}

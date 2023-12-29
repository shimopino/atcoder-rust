use proconio::input;

fn main() {
    input! {
        n: String,
    }

    println!("{}", u32::from_str_radix(n.as_str(), 2).unwrap());
}

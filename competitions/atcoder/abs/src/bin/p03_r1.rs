use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; n]
    };
    let result = {
        a.iter()
         .map(|&x| x.trailing_zeros())
         .min()
         .unwrap()
    };
    println!("{}", result);
}
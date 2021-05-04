use proconio::input;

fn main() {
    input! {
        md: [(u32, u32); 2],
    }

    let ans = md[1].0 - md[0].0;
    println!("{}", ans);
}
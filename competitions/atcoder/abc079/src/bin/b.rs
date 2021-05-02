use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut l0: u64 = 2;
    let mut l1: u64 = 1;
    for _ in 2..=n {
        let tmp = l0 + l1;
        l0 = l1;
        l1 = tmp;
    }
    println!("{}", l1); 
}
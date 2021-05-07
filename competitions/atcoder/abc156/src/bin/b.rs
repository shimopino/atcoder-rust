use proconio::input;

fn main() {
    input! {
        mut n: u32,
        k: u32,
    }
    
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= k;
    }
    println!("{}", count);
}
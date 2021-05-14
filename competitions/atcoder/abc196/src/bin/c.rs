use proconio::input;

fn main() {
    input! {
        mut n: u128,
    }

    let mut count = 0;
    for i in 1..(1000000+1) {
        let s = format!("{}{}", i,i);
        if n >= s.parse::<u128>().unwrap() {
            count += 1;
        }
    }

    println!("{}", count);
}
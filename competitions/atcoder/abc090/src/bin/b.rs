use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    let mut count = 0;
    for i in a..b + 1 {
        let number = i.to_string();
        let rev_number: String = number.chars().rev().collect();
        if number == rev_number {
            count += 1;
        }
    }
    println!("{}", count);
}
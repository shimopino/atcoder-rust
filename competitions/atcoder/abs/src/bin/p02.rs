use proconio::input;

fn main() {
    input! {
        s: String
    };
    let mut result: i32 = 0;
    for ichar in s.chars() {
        if ichar == '1' {
            result += 1;
        }
    }
    println!("{}", result);
}
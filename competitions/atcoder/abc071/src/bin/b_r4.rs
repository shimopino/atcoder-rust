use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        s: Bytes,
    }
    for b in 0..26 {
        let c = std::char::from_u32('a' as u32 + b).unwrap();
        if s.iter().all(|p| *p != c as u8) {
            println!("{}", c);
            return;
        }
    }
    println!("None");
}
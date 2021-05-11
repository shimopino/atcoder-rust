use proconio::input;

fn main() {
    input! {
        s: String,
    }

    for i in 0..26 {
        let c = std::char::from_u32('a' as u32 + i).unwrap();
        if !s.contains(c) {
            println!("{}", c);
            return;
        }
    }
    println!("None");
}
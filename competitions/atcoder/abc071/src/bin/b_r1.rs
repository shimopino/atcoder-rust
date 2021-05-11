use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut found = vec![false; 26];
    for c in s.chars() {
        found[(c as u8 - 'a' as u8) as usize] = true;
    }
    for i in 0..26 {
        if !found[i] {
            println!("{}", (i as u8 + 'a' as u8) as char);
            return;
        }
    }
    println!("None");
}

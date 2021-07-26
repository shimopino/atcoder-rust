use proconio::input;

fn main() {
    input! {
        mut s: [String; 4],
    }

    s.sort();
    s.dedup();

    println!(
        "{}", if s.len() == 4 {
            "Yes"
        } else {
            "No"
        }
    )
}

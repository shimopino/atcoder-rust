use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let s_vec: Vec<char> = s.chars().collect();
    println!("{}{}{}", s_vec[1], s_vec[2], s_vec[0]);
}

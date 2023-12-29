use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut found = false;
    for i in 0..a.len() - 2 {
        for j in i + 1..a.len() - 1 {
            if a[j + 1..].contains(&(1000 - a[i] - a[j])) {
                found = true;
            }
        }
    }

    println!("{}", if found { "Yes" } else { "No" });
}

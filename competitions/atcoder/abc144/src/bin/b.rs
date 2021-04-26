use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut flag = false;
    for i in 1..10 {
        for j in 1..10 {
            if i * j == n {
                flag = true;
            }
        }
    }
    println!("{}", if flag { "Yes" } else { "No" });
}

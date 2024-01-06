use proconio::input;

fn main() {
    input! {
        n: i32
    }

    for i in 0..=21 {
        for j in 0..=21 {
            for k in 0..=21 {
                let total = i + j + k;
                if total <= n {
                    println!("{i} {j} {k}");
                }
            }
        }
    }
}

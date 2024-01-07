use proconio::input;

fn main() {
    input! {
        n: i32
    }

    for x in 0..=21 {
        for y in 0..=21 {
            for z in 0..=21 {
                if x + y + z <= n {
                    println!("{x} {y} {z}");
                }
            }
        }
    }
}

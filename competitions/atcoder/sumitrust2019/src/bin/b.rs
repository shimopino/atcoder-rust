use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    for i in 1..50001 {
        if (i as f32 * 1.08) as u32 == n {
            println!("{}", i);
            return;
        }
    }

    println!(":(");
}
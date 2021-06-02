use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    for i in 1..50001 {
        let original = i as f32;
        if (original * 1.08) as i32 == n {
            println!("{}", i);
            return;
        }
    }
    println!(":(")
}
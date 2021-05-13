use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    }

    let ab: String = format!("{}{}", a, b);
    let n = ab.parse::<u32>().unwrap();
    
    for i in 1..400 {
        if i * i == n {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
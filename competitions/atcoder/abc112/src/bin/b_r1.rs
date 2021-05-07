use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u32,
        ct: [(u32, u32); n]
    }

    let min_cost = ct.iter()
                     .filter(|ict| ict.1 <= t)
                     .map(|ict| ict.0)
                     .min();
    
    match min_cost {
        None => println!("TLE"),
        Some(x) => println!("{}", x),
    }
}
use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        w: i32,
    }

    let w_gram = w * 1000;
    let mut count_min = std::i32::MAX;
    let mut count_max = -1;

    for n in 1..=1_000_000 {
        if a * n <= w_gram && w_gram <= b * n {
            count_min = count_min.min(n);
            count_max = count_max.max(n);
        }
    }
    
    if count_max == -1 {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", count_min, count_max);
    }
}
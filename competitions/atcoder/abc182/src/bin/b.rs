use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let maximum = a.iter().max().unwrap();

    let mut previous_count = 0;
    let mut ans = 0;
    for i in 2..=*maximum {
        let mut count = 0;
        for j in 0..n {
            if a[j] % i as u32 == 0 {
                count += 1;
            }
        }
        if count >= previous_count {
            previous_count = count;
            ans = i;
        }
    }
    println!("{}", ans);
}

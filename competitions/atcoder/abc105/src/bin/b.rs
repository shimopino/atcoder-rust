use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut count = 0;
    for i in 1..=n {
        // 奇数のみ探索
        if i % 2 == 1 {
            let mut yakusu_count = 0;
            for j in 1..=i {
                if i % j == 0 {
                    yakusu_count += 1;
                }
            }
            if yakusu_count == 8 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
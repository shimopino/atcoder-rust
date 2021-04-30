use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let mut biggest_count = -1;
    let mut biggest = -1; 
    for inumber in 1..=n {
        let mut count = 0;
        let mut target = inumber;
        while target % 2 == 0 {
            count += 1;
            target /= 2;
        }

        if count > biggest_count {
            biggest_count = count;
            biggest = inumber;
        }
    }
    println!("{}", biggest);
}
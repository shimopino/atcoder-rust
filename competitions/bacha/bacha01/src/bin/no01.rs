use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut minimum = std::i32::MAX;
    for ia in a {
        let mut i = ia;
        let mut count = 0;
        while i % 2 == 0 {
            count += 1;
            i /= 2;
        }

        minimum = minimum.min(count);
    }
    println!("{}", minimum);
}

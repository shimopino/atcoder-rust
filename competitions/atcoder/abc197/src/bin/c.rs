use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut ans = std::i32::MAX;
    for bit in 0..(1 << (n - 1)) {
        let mut or = 0;
        let mut xor = 0;

        for (i, &a) in a.iter().enumerate() {
            or |= a;
            if bit & (1 << i) == 0b0 {
                xor ^= or;
                or = 0;
            }
        }
        ans = ans.min(xor ^ or);
    }
    println!("{}", ans);
}
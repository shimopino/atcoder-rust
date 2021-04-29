use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [u32; n],
    }

    let mut maximum = 201;
    for bit in 0..(1 << n) {
        let mut s1 = Vec::new();
        let mut s2 = Vec::new();
        for i in 0..n {
            if bit & (1 << i) != 0b0 {
                s1.push(t[i]);
            } else {
                s2.push(t[i]);
            }
        }

        let s_maximum = s1.iter().sum::<u32>().max(s2.iter().sum::<u32>());
        if s_maximum < maximum {
            maximum = s_maximum;
        }
    }
    println!("{}", maximum);
}
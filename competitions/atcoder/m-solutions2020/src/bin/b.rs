use proconio::input;

fn main() {
    input! {
        mut a: u32,
        mut b: u32,
        mut c: u32,
        K: u32,
    }

    let mut flag = false;
    for i in 0..=K {
        for j in 0..=K {
            for k in 0..=K {
                let x = a * (1 << i);
                let y = b * (1 << j);
                let z = c * (1 << k);

                if i + j + k <= K && x < y && y < z {
                    flag = true;
                }
            }
        }
    }
    println!("{}", if flag { "Yes" } else { "No" })
}

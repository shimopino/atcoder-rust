use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        K: u32,
    }

    let mut flag = false;
    let base: u32 = 2;
    for i in 0..K + 1 {
        for j in 0..K + 1 {
            for k in 0..K + 1 {
                let red = a * base.pow(i);
                let green = b * base.pow(j);
                let blue = c * base.pow(k);

                if i + j + k <= K && blue > green && green > red {
                    flag = true;
                }
            }
        }
    }
    println!("{}", if flag { "Yes" } else { "No" });
}

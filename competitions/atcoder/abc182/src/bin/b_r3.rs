use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut ans = 0;
    let mut count = 0;
    for i in 2..1001 {
        let mut c = 0;
        for an in a.iter() {
            if an % i == 0 {
                c += 1;
            }
        }

        if c > count {
            count = c;
            ans = i;
        }
    }
    println!("{}", ans);
}
use proconio::input;

fn main() {
    input! {
        mut a: u32,
        mut b: u32,
        mut c: u32,
        k: u32,
    }

    // c > b > a にするために必要な回数を数え上げる
    let mut count = 0;
    while b <= a {
        b *= 2;
        count += 1;
    }

    while c <= b {
        c *= 2;
        count += 1;
    }

    println!("{}", if count <= k { "Yes" } else { "No" });
}

use proconio::input;

fn main() {
    input! {
        a: usize,
        mut b: usize,
        mut c: usize,
        k: usize
    }

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

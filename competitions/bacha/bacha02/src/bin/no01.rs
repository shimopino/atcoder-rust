use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut maximum = 0;
    for i in 0..n {
        for j in i+1..n {
            let dist = (a[i] - a[j]).abs();
            maximum = maximum.max(dist);
        }
    }
    println!("{}", maximum);
}

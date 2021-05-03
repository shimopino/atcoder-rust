use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }

    let mut alice = 0;
    let mut bob = 0;
    
    a.sort();
    a.reverse();

    for i in 0..a.len() {
        if i % 2 == 0 {
            alice += a[i];
        } else {
            bob += a[i];
        }
    }
    println!("{}", alice - bob);
}

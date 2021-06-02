use proconio::input;

fn main() {
    input! {
        n: usize,
        mut l: [u32; n],
    }

    l.sort();
    
    let mut count = 0;
    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                if l[i] != l[j] && l[j] != l[k] && l[i] + l[j] > l[k] {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
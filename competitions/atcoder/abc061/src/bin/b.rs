use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [[usize; 2]; m],
    }

    let mut ans: Vec<u32> = vec![0; n];
    for iab in ab.iter() {
        ans[iab[0] - 1] += 1;
        ans[iab[1] - 1] += 1;
    }
    
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
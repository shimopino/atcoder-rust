use proconio::{input, fastout, marker::Usize1};
 
#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        pair: [(Usize1, usize); m],
    }
 
    let mut mm: Vec<Vec<usize>> = vec![vec![]; n];
    for &(p, y) in &pair {
        mm[p].push(y);
    }
 
    for v in mm.iter_mut() {
        v.sort();
    }
 
    for &(p, y) in &pair {
        let index = mm[p].binary_search(&y).unwrap();
        println!("{:06}{:06}", p+1, index+1);
    }
}
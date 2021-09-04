use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(i64, i64); n],
        q: usize,
        lr: [(i64, i64); q],
    }

    let mut acm_a = vec![0_i64; n + 1];
    let mut acm_b = vec![0_i64; n + 1];

    for (i, (c, p)) in cp.iter().enumerate() {
        acm_a[i + 1] += acm_a[i];
        acm_b[i + 1] += acm_b[i];

        // 分岐の数が少ない場合 は、if-else文よりも、複数のif文の方が綺麗
        if *c == 1 { acm_a[i + 1] += *p } 
        if *c == 2 { acm_b[i + 1] += *p }
    }

    // println!("{:?}", acm_a);
    // println!("{:?}", acm_b);

    for (_i, (l, r)) in lr.into_iter().enumerate() {
        let left = l - 1;
        let total_a = acm_a[r as usize] - acm_a[left as usize];
        let total_b = acm_b[r as usize] - acm_b[left as usize];

        print!("{} {}", total_a, total_b);
        println!("")
    }

    return;
}
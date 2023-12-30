use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut sabun = vec![0; t + 1];
    for &(l, r) in lr.iter() {
        sabun[l] += 1;
        sabun[r] -= 1;
    }

    let mut accum = vec![0; t + 1];
    for (index, &isabun) in sabun.iter().enumerate() {
        accum[index] += isabun + if index == 0 { 0 } else { accum[index - 1] }
    }

    for index in 0..t {
        println!("{}", accum[index]);
    }
}

use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut sabun = vec![0; d];
    for &(l, r) in lr.iter() {
        sabun[l - 1] += 1;
        if r < sabun.len() {
            sabun[r] -= 1;
        }
    }

    let mut accum = vec![0; d];
    for (index, &isabun) in sabun.iter().enumerate() {
        accum[index] += isabun + if index == 0 { 0 } else { accum[index - 1] }
    }

    for &npersons in accum.iter() {
        println!("{}", npersons);
    }
}

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    }

    let mut h_total = vec![0; h];
    let mut w_total = vec![0; w];

    for ih in 0..h {
        let mut total = 0;
        for iw in 0..w {
            total += a[ih][iw]
        }

        h_total[ih] = total;
    }

    for iw in 0..w {
        let mut total = 0;
        for ih in 0..h {
            total += a[ih][iw]
        }

        w_total[iw] = total;
    }

    for ih in 0..h {
        for iw in 0..w {
            let answer = h_total[ih] + w_total[iw] - a[ih][iw];

            print!("{} ", answer);
       }
       println!();
    }

    return;
}
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; h]; w],
    }

    let mut row = vec![0; h];
    let mut column = vec![0; w];

    for ih in 0..h {
        for iw in 0..w {
            row[ih] += a[ih][iw];
            column[iw] += a[ih][iw];
        }
    }

    let mut answer = vec![vec![0; h]; w];

    for ih in 0..h {
        for iw in 0..w {
            answer[ih][iw] = row[ih] + column[iw] - a[ih][iw];
        }
    }

    for ih in 0..h {
        for iw in 0..w {
            print!("{} ", answer[ih][iw]);
        }
        println!()
    }

    return;
}
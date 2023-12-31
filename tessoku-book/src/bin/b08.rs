use proconio::input;

const MAX: usize = 1500;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }

    // 各軸の最大値と累積和の初期化
    let mut accum = vec![vec![0; MAX + 1]; MAX + 1];

    // 各座標に、XYの出現地点を追加
    for &(x, y) in xy.iter() {
        accum[y][x] += 1;
    }

    // 縦方向の累積和
    for iw in 0..MAX {
        for ih in 0..MAX {
            accum[ih + 1][iw + 1] += accum[ih + 1][iw];
        }
    }

    // 縦方向の累積和
    for ih in 0..MAX {
        for iw in 0..MAX {
            accum[ih + 1][iw + 1] += accum[ih][iw + 1];
        }
    }

    for &(a, b, c, d) in abcd.iter() {
        let total = accum[d][c] - accum[d][a - 1] - accum[b - 1][c] + accum[b - 1][a - 1];
        println!("{}", total);
    }
}

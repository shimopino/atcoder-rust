use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[i64; w]; h],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }

    // 累積配列の初期化
    // 縦横の長さを1つ伸ばして、端の座標でも同じ計算式を使えるようにする
    let mut accum = vec![vec![0; w + 1]; h + 1];

    // 横方向の累積和を計算する
    for ih in 0..h {
        for iw in 0..w {
            accum[ih + 1][iw + 1] += x[ih][iw] + accum[ih + 1][iw]
        }
    }

    // 縦方向の累積和を計算する
    for iw in 0..w {
        for ih in 0..h {
            accum[ih + 1][iw + 1] += accum[ih][iw + 1]
        }
    }

    for &(a, b, c, d) in abcd.iter() {
        let total = accum[c][d] + accum[a - 1][b - 1] - accum[c][b - 1] - accum[a - 1][d];
        println!("{}", total);
    }
}

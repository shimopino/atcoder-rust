use proconio::input;

// wrapping_add を利用すれば usize でも -1 の計算が可能となる
// 時計回りに90度回転させていく
const DX: [usize; 4] = [1, 0, !0, 0];
const DY: [usize; 4] = [0, 1, 0, !0];

fn main() {
    input! {
        n: usize,
    }

    let mut grid = vec![vec![0; n]; n];
    let mut direction = 0;
    let mut x = 0;
    let mut y = 0;

    for i in 1..=n.pow(2) {
        // 探索する順番に従ってインクリメントさせた数値を代入する
        grid[x][y] = i;
        // 次に移動する座標を計算する
        let dx = x.wrapping_add(DX[direction]);
        let dy = y.wrapping_add(DY[direction]);

        // マイナス側への領域は、 wrapping_add で最大値として計算される
        // そのため、上下左右の方向に対して境界の「n」を超える
        if dx >= n || dy >= n || grid[dx][dy] != 0 {
            // 90度で回転して４回で１周する
            // これは mod 4 で計算できる
            direction += 1;
            direction %= 4;
        }
        x = x.wrapping_add(DX[direction]);
        y = y.wrapping_add(DY[direction]);
    }

    for y in 0..n {
        for x in 0..n {
            if grid[x][y] == n * n {
                print!("T ");
                continue;
            }
            print!("{} ", grid[x][y]);
        }
        println!();
    }
}

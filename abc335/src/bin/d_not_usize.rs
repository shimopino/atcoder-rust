use proconio::input;

// wrapping_add を利用すれば usize でも -1 の計算が可能となる
// 時計回りに90度回転させていく
const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

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
        grid[x as usize][y as usize] = i;
        // 次に移動する座標を計算する
        let dx = x + DX[direction];
        let dy = y + DY[direction];

        let upper = n as i32;
        // マイナス側への領域は、 wrapping_add で最大値として計算される
        // そのため、上下左右の方向に対して境界の「n」を超える
        if dx >= upper || dy >= upper || dx <= -1 || dy <= -1 || grid[dx as usize][dy as usize] != 0
        {
            // 90度で回転して４回で１周する
            // これは mod 4 で計算できる
            direction += 1;
            direction %= 4;
        }
        x += DX[direction];
        y += DY[direction];
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

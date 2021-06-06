use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        field: [Chars; h],
    }

    // 探索済みのノードを配列 seen で管理する
    let mut seen = vec![vec![false; w]; h];

    // 開始マス目を特定する
    let mut start = (0, 0);
    for ih in 0..h {
        for iw in 0..w {
            if field[ih][iw] == 's' {
                start = (ih, iw);
            }
        }
    }

    let mut stack = Vec::new();
    stack.push(start);

    // 開始地点を true に設定する
    seen[start.0][start.1] = true;

    while let Some(top) = stack.pop() {
        let y = top.0;
        let x = top.1;

        // 4方向を全て探索する
        for (dy, dx) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
            // 次の探索先を計算する
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;

            // 探索先が場外の場合は無視する
        }
    }
}
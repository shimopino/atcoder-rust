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

    // 開始と終了マス目を特定する
    let mut sh = 0;
    let mut sw = 0;
    let mut gh = 0;
    let mut gw = 0;
    for ih in 0..h {
        for iw in 0..w {
            if field[ih][iw] == 's' {
                sh = ih;
                sw = iw;
            }
            if field[ih][iw] == 'g' {
                gh = ih;
                gw = iw;
            }
        }
    }

    let mut stack = Vec::<(usize, usize)>::new();
    stack.push((sh, sw));

    // 開始地点を true に設定する
    seen[sh][sw] = true;

    while let Some(top) = stack.pop() {
        let y = top.0;
        let x = top.1;

        // 4方向を全て探索する
        for (dy, dx) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
            // 次の探索先を計算する
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;

            // 探索先が場外の場合は無視する
            if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 {
                continue;
            }

            let ny = ny as usize;
            let nx = nx as usize;

            // 探索済みの場合は無視する
            if seen[ny][nx] {
                continue;
            }

            // 探索先が壁の場合は無視する
            if field[ny][nx] == '#' {
                continue;
            }

            // 未訪問の場合は探索済みにして、次の探索先を追加
            seen[ny][nx] = true;
            stack.push((ny, nx));
        }
    }

    println!("{}", if seen[gh][gw] { "Yes" } else { "No" });
}

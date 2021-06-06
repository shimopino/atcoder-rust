use proconio::input;
use proconio::marker::Chars;

// 右, 下, 左, 上
const DXY: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    input! {
        h: usize,
        w: usize,
        field: [Chars; h],
    }

    // start と goal のマス目位置を特定する
    let mut sh = 0;
    let mut sw = 0;
    let mut gh = 0;
    let mut gw = 0;
    for ih in 0..h {
        for jw in 0..w {
            if field[ih][jw] == 's' {
                sh = ih as i32;
                sw = jw as i32;
            }
            if field[ih][jw] == 'g' {
                gh = ih as i32;
                gw = jw as i32;
            }
        }
    }

    // seen配列全体を false で初期化
    let mut seen = vec![vec![false; w]; h];

    dfs(sh, sw, &mut seen, &field);

    println!(
        "{}",
        if seen[gh as usize][gw as usize] {
            "Yes"
        } else {
            "No"
        }
    );
}

fn dfs(h: i32, 
       w: i32, 
       seen: &mut std::vec::Vec<std::vec::Vec<bool>>,
       field: &std::vec::Vec<std::vec::Vec<char>>) {

    // 訪れたマスは訪問済みとして true にする
    seen[h as usize][w as usize] = true;

    let h_max: i32 = seen.len() as i32 - 1;
    let w_max: i32 = seen[0].len() as i32 - 1;

    // 4方向を探索
    for dir in 0..4 {
        // 次の移動先のマスを計算
        let nh = h + DXY[dir].0;
        let nw = w + DXY[dir].1;

        // 探索範囲が場外の場合はスルー
        if nh < 0 || nw < 0 || nh > h_max || nw > w_max {
            continue;
        }

        // 移動先が壁の場合はスルー
        if field[nh as usize][nw as usize] == '#' {
            continue;
        }

        // 移動先が探索済みの場合はスルー
        if seen[nh as usize][nw as usize] {
            continue;
        }

        dfs(nh, nw, seen, &field);
    }
}

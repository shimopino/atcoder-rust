use proconio::input;
use proconio::marker::Chars;

/// 戦略を残す
///
/// * 陸地がつながっていること
///   * 陸地に変換するマスを1つ選択する
///   * 変換したマスを根とする深さ優先探索を行う
///   * 探索箇所は "x" に変換する
///   * もしも陸地が全てつながっている場合、全マスが "x" となるはず
fn main() {
    input! {
        field: [Chars; 10],
    }

    // 陸地がつながっているパターンがあれば true にする
    let mut ok = false;
    // 陸地に変換する場所を全パターン試す
    for y in 0..10 {
        for x in 0..10 {
            // 対象マスを陸地 ("o") に変換する
            let mut tmp_field = field.clone();
            tmp_field[y][x] = 'o';

            // 対象マスを起点につながっている陸地を全て海 ("x") に変換する
            dfs(&mut tmp_field, y, x);

            // 対象エリアが全て海になった場合は繋がっている
            if all_x(&tmp_field) {
                ok = true;
            }
        }
    }

    println!("{}", if ok { "YES" } else { "NO" });
}

fn dfs(field: &mut Vec<Vec<char>>, y: usize, x: usize) {
    // 探索対象マスを海 ("x") に変換する
    field[y][x] = 'x';

    // 4方向全てを探索する
    for (dy, dx) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
        // 次の探索先の決定
        let ny = y as i32 + dy;
        let nx = x as i32 + dx;

        // 探索先が場外の場合はスルー
        if ny < 0 || nx < 0 || ny == 10 || nx == 10 {
            continue;
        }

        let ny = ny as usize;
        let nx = nx as usize;

        // 次の探索先が陸地 ("o") の場合は探索を継続する
        if field[ny][nx] == 'o' {
            dfs(field, ny, nx);
        }
    }
}

/// 陸地が繋がっているのか判定する
/// 全てが海 ('x') の場合は繋がっていたと判定して true を返す
/// 
/// * `field` - 探索済みの全マス
fn all_x(field: &Vec<Vec<char>>) -> bool {
    for y in 0..10 {
        for x in 0..10 {
            // 陸地が1つでもあれば繋がっていない
            if field[y][x] != 'x' {
                return false;
            }
        }
    }
    true
}

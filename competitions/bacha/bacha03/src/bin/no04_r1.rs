use proconio::input;

/// 戦略
///   深さ優先探索
///   次のノードを探索する際に、自身のノードのカウンター値を渡す
///   この操作を続けていくことで、ノードを深堀りしていけば、次の
///   ノードにカウンター値の累積値が渡されていく
/// 
///   グラフに自体は無向グラフで作成し、親ノードの場合は無視する
/// 
///   今回は counter に直接累積していく方針をとる
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        px: [(usize, usize); q],
    }

    // まずは無向グラフを構築する
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    // クエリで指定された根とカウンター値を作成する
    let mut counter = vec![0; n];
    for (p, x) in px {
        counter[p - 1] += x;
    }

    // 根は1なので配列に合わせて探索開始頂点を 0 に設定
    dfs(0, 0, &mut counter, &graph);
    
    for i in 0..n {
        print!("{} ", counter[i]);
    }
    println!("");
}

/// Args
///   * current: 現在の根
///   * parent: 親の根
///   * counter: 各頂点のカウンター値
///   * graph: 構築済みの無向グラフ
fn dfs(
    current: usize,
    parent: usize,
    counter: &mut Vec<usize>,
    graph: &Vec<Vec<usize>>
) {
    // 探索中のノードに紐づく全ノード (親を含む) を探索
    for child in &graph[current] {
        // 次の探索先が子のノードの場合のみ探索する
        if *child != parent {
            // 自身のカウンター値を、子ノードに渡す
            counter[*child] += counter[current];
            dfs(*child, current, counter, graph);
        }
    }
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for (u, v) in uv {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }

    // 探索済みの頂点を保存する
    let mut visited = vec![false; n];

    let mut ans = 0;
    for i in 0..n {
        if dfs(&mut visited, &graph, i, i) {
            // DFSで閉路判定されない場合にのみカウントアップ
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn dfs(
    visited: &mut Vec<bool>,
    graph: &Vec<Vec<usize>>,
    current: usize,
    parent: usize,
) -> bool {

    // もしも探索先が、探索済みの場合は閉路判定をする
    if visited[current] {
        return false;
    }

    visited[current] = true;

    for &child in &graph[current] {
        if child != parent {
            // 親ノードへの探索は実施しない
            if !dfs(visited, graph, child, current) {
                return false;
            }
        }
    }

    return true;
}
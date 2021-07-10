use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        cd: [(usize, usize); q],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let mut dist = vec![-1; n];
    let mut stack = vec![];
    
    // 街1を根として二部グラフを構築する
    dist[0] = 0;
    stack.push(0);
    while !stack.is_empty() {
        let current: usize = stack.pop().unwrap();
        for &next in &graph[current] {
            if dist[next] == -1 {
                dist[next] = dist[current] + 1;
                stack.push(next);
            }
        }
    }

    for (c, d) in cd {
        let start = c - 1;
        let end = d - 1;

        if dist[start] % 2 == dist[end] % 2 { println!("Town") }
        if dist[start] % 2 != dist[end] % 2 { println!("Road") }
    }
}

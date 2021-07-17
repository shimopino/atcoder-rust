<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
<details>
<summary>Table of Contents</summary>

- [ABC209](#abc209)
  - [A問題](#a問題)
  - [B問題](#b問題)
  - [C問題](#c問題)
  - [D問題](#d問題)

</details>
<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# ABC209

## A問題

整数の大小を判定して、閉区間内の整数の数を計算すればいい。

```rust
use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    println!(
        "{}",
        if b >= a {
            b - a + 1
        } else {
            0
        }
    );
}
```

## B問題

偶数商品と奇数商品で計算方法を分けて合計金額を計算し、所持している金額以下であることを判定すればいい。

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut count = 1;
    let mut total = 0;
    for ia in a {
        if count % 2 == 0 {
            total += ia - 1;
        } else {
            total += ia;
        }
        count += 1;
    }

    println!(
        "{}",
        if total <= x {
            "Yes"
        } else {
            "No"
        }
    )
}

```

## C問題

昇順に並び替えてしまえば、ループ内のカウント数を減算した数が場合の数となるので、ループごとに計算した場合の数をかけていけばいい。

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c: [u128; n],
    }

    c.sort();
    let mut total: u128 = 1;
    for (i, ic) in c.into_iter().enumerate() {
        total *= ic - i as u128;
        total %= 1_000_000_000 + 7;
    }

    println!("{}", total);
}

```

## D問題

2部グラフを構築し、始点と終点が偶数に所属しているのか、奇数に所属しているのか判定すればいい。

```rust
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
```

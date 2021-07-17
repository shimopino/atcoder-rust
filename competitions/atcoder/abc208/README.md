<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
<details>
<summary>Table of Contents</summary>

- [ABC208](#abc208)
  - [A問題](#a問題)
  - [B問題](#b問題)
  - [C問題](#c問題)
  - [D問題](#d問題)
  - [E問題](#e問題)
  - [F問題](#f問題)

</details>
<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# ABC208

## A問題

1mLあたりのkcalを計算して、BmL分のkcalを計算すればいい。

```rust
fn main() {
    proconio::input! {
        a: f64, b: f64,
    }

    let ans = a * b / 100_f64;
    println!("{}", ans);
}
```

> 反省点
> 整数除算をしてしまいWrong Answerになってしまった...

## B問題

与えられた整数の数列を昇順に並び替えて、配列の添え字に1を足した値と一致しているかどうか判定する。

```rust
fn main() {
    proconio::input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut ans = true;
    for (i, ia) in a.iter().enumerate() {
        if i + 1 != *ia {
            ans = false;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
```

## C問題

与えられた整数A、Bの正負に応じて計算を分ける必要がある。

- Aが正、Bが正
  - 整数の大小をそのまま計算できる
- AとBのどちらかが正、どちらかが負
  - 累乗Cが偶数の場合は、整数の絶対値の大小関係が成立する
  - 累乗Cが奇数の場合は、整数の大小関係が成立する
- Aが負、Bが負
  - 累乗Cが偶数の場合は、整数の絶対値の大小関係が成立する
  - 累乗Cが奇数の場合は、整数の大小関係が成立する

与えられた整数の数列を昇順に並び替えて、配列の添え字に1を足した値と一致しているかどうか判定する。

```rust
fn main() {
    proconio::input! {
        a: i128,
        b: i128,
        c: i128,
    }

    // 累乗が奇数か偶数かで結果は変わる
    if c % 2 == 0 {
        // 偶数
        if a.abs() > b.abs() {
            println!(">");
            return;
        } else if a.abs() < b.abs() {
            println!("<");
            return;
        } else {
            println!("=");
            return;
        }
    } else {
        // 奇数
        if a > b {
            println!(">");
            return;
        } else if a < b {
            println!("<");
            return;
        } else {
            println!("=");
            return;
        }
    }
}
```

公式解説放送では、偶数の場合に絶対値に変換することで、条件判定を共通化しており、コチラの方法だとよりコードが短くなる。

```rust
use proconio::input;

fn main() {
    proconio::input! {
        mut a: i128,
        mut b: i128,
        c: i128,
    }

    if c % 2 == 0 {
        a = a.abs();
        b = b.abs();
    }

    if a > b { println(">"); return; }
    if a == b { println("="); return; }
    if a < b { println(">"); return; }
}
```

## D問題

正整数列の隣り合う各要素に対して、その区間内に収まる整数がいくつあるのかを計算する。

あとは計算された区間内に収まる整数の数の累積和を計算し、クエリKに対して該当する区間を探索し、

```rust
fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        a: [u128; n],
        k: [u128; q],
    }

    let mut pre = vec[0; n + 1];
    for i in 0..n {
        pre[i+1] = a[i] - i - 1;
    }

    
}
```

## E問題

## F問題

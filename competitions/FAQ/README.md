# 競プロに関するFAQ

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
<details>
<summary>Table of Contents</summary>

- [Q1 `proconio` を使用して標準入力から空白区切りで値を取得するには](#q1-proconio-%E3%82%92%E4%BD%BF%E7%94%A8%E3%81%97%E3%81%A6%E6%A8%99%E6%BA%96%E5%85%A5%E5%8A%9B%E3%81%8B%E3%82%89%E7%A9%BA%E7%99%BD%E5%8C%BA%E5%88%87%E3%82%8A%E3%81%A7%E5%80%A4%E3%82%92%E5%8F%96%E5%BE%97%E3%81%99%E3%82%8B%E3%81%AB%E3%81%AF)
- [Q2 値が偶数か奇数かに応じて文字列を返す処理を1行で表現するには](#q2-%E5%80%A4%E3%81%8C%E5%81%B6%E6%95%B0%E3%81%8B%E5%A5%87%E6%95%B0%E3%81%8B%E3%81%AB%E5%BF%9C%E3%81%98%E3%81%A6%E6%96%87%E5%AD%97%E5%88%97%E3%82%92%E8%BF%94%E3%81%99%E5%87%A6%E7%90%86%E3%82%921%E8%A1%8C%E3%81%A7%E8%A1%A8%E7%8F%BE%E3%81%99%E3%82%8B%E3%81%AB%E3%81%AF)
- [QN](#qn)

</details>
<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Q1 `proconio` を使用して標準入力から空白区切りで値を取得するには

<details>
<summary>回答</summary>

入力は以下の場合を考える。

```bash
1 10
```

入力を受け取るには以下を実行する。

```rust
use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    };
    let result = a + b;
    println!("{}", result);
}
```

</details>

## Q2 値が偶数か奇数かに応じて文字列を返す処理を1行で表現するには

<details>
<summary>回答</summary>

複数行の場合には以下のように記述する。

```rust
fn main() {
    input! {
        a: i32,
        b: i32
    };
    let result = {
        if (a * b) % 2 == 0 {
            "Even"
        } else {
            "Odd"
        }
    };
    println!("{}", result);
}
```

1行で以下のように記述できる。

```rust
fn main() {
    input! {
        a: i32,
        b: i32
    };
    let result = if (a * b) % 2 == 0 { "Even" } else {"Odd"};
    println!("{}", result);
}
```

</details>

## QN

<details>
<summary>回答</summary>


</details>

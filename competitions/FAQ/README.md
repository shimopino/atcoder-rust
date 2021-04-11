# 競プロに関するFAQ

<!-- START doctoc -->
<!-- END doctoc -->

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

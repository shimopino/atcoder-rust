# 数当てゲーム

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
<details>
<summary>Table of Contents</summary>

- [Q1](#q1)
- [Q2](#q2)
- [QN](#qn)

</details>
<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Q1

Rustで `quess` という変数を、不変で定義する方法と可変で定義する方法はどのようなものでしょうか。

`guess` に空の文字列を割り当てるコードで実践してください。

<details>
<summary>回答例</summary>

Rustでは標準で変数は不変 (`immutable`) で定義される。

```rust
let guess = String::new();
```

可変 (`mutable`) で定義したい場合には変数宣言に `mut` を付ける必要がある。

```rust
let mut guess = String::new();
```

</details>

## Q2

可変の変数であり、空の文字列が代入されている変数 `guess` に対して、ユーザが標準入力に入力した値を割り当てるにはどうすればいいでしょうか。

<details>
<summary>回答例</summary>

まずは変数が以下の形で定義されているとする。

```rust
let mut guess = String::new();
```

標準入力を使用するには、標準ライブラリ `std` の入出力ライブラリ `io` をスコープに含める必要がある。

```rust
use std::io;
```

標準入力は、入出力ライブラリ `io` に定義されている `stdin` という静的メソッドの、`read_line` を使用する。

```rust
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

`&` を使用することで宣言済みの変数の参照を取得することができる。また参照もデフォルトで不変なので、可変にして変数にデータを格納している。

`expect` を使用することで、`real_line()` が `Err` 列挙子を返した場合に、引数の値を出力してプログラムをクラッシュさせることができる。
 
</details>

## QN

<details>
<summary>回答例</summary>
</details>
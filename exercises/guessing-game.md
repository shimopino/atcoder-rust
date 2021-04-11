# 数当てゲーム

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
<details>
<summary>Table of Contents</summary>

- [Q1](#q1)
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

## QN

<details>
<summary>回答例</summary>
</details>
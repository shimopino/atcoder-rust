# 数当てゲーム

<!-- START doctoc -->
<!-- END doctoc -->

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
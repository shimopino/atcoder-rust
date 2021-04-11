# 所有権

<!-- START doctoc -->
<!-- END doctoc -->

## Q1

以下のコードに関して変数 `s` のスコープはどこからどこまででしょうか。

```rust
{
    let s = "hello";
}
```

<details>
<summary>回答例</summary>

```rust
{                       // 無効。宣言されていない
    let s = "hello";    // 有効
}                       // 無効。スコープの終了
```

</details>

## QN

<details>
<summary>回答例</summary>



</details>

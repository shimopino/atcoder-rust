# Hello World

<!-- START doctoc -->
<!-- END doctoc -->

## Q1

インストールしているRustのバージョン情報をコマンドラインから確認するコマンドは何でしょうか

<details>
<summary>回答例</summary>

```sql
$ rustc --version
rustc 1.51.0 (2fd73fabe 2021-03-23)
```

</details>

## Q2

[playground/helloworld](../playground/helloworld)直下に `main.rs` というファイルを作成し、`Hello World!`と出力させるプログラムを作成して実行結果を確認してください。

<details>
<summary>回答例</summary>

まずは `main.rs` を作成する。

```rs
fn main() {
    // こんにちは！！
    println!("Hello World!");
}
```

次にプログラムをコンパイルし、実行する。

```bash
$ rustc main.rs
$ ./main
Hello World!
```

</details>

## QN

<details>
<summary>回答例</summary>
</details>

## 参考情報

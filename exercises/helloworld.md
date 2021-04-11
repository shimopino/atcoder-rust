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

```rust
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

## Q3

Rustのビルドシステム、パッケージマネージャを担う `Cargo` を使用して、新規のプロジェクト `hello_cargo` を [playground/helloworld](../playground/helloworld) 直下に作成してみましょう。

また作成後にどのようなディレクトリ構造になっているのか確認してみましょう。

<details>
<summary>回答例</summary>

実行可能なバイナリを作成し、新規のプロジェクトを作成する。

```bash
$ cargo new hello_cargo --bin
$ tree hello_cargo

hello_cargo/
├── Cargo.toml
└── src
    └── main.rs
```

作成されている `Cargo.toml` は、以下のように作成しているパッケージに関する情報が記載される `[project]` と、プロジェクトが依存しているパッケージが列挙される `[dependancies]` で構成されていることがわかる。

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["KeisukeShimokawa <shimoke4869@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

`src/main.rs` には以下のように文字列を出力する単純なファイルが作成されていることがわかる。

```rust
fn main() {
    println!("Hello World!");
}
```

</details>

## Q4

[playground/helloworld](../playground/helloworld) 直下に作成した新規プロジェクト `hello_cargo` をビルドし、実行してみましょう。

またその際に、ビルド前にコンパイルできるか確認するためのコマンドを実行してみましょう。

<details>
<summary>回答例</summary>

まずは `cargo check` でコードがコンパイル可能か確認する。

```bash
$ cargo check
    Checking hello_cargo v0.1.0 (/workspaces/rust-exercises/playground/helloworld/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
```

では次にプロジェクトのビルドを実行する。

```bash
$ cargo build
   Compiling hello_cargo v0.1.0 (/workspaces/rust-exercises/playground/helloworld/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
```

ビルドを実行した結果、以下のように新たなファイルが作成されていることがわかる。

```bash
$ tree -L 2

hello_cargo/
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
```

最上位のディレクトリに作成される `Cargo.lock` には、プロジェクトが依存しているパッケージの正確なバージョン情報を管理している。

また `target/debug/` に開発用にコンパイルされた実行可能なファイルが作成されており、`target/debug/hello_world` という名称でバイナリファイルが作成されている。

では次に実行可能ファイルを実行する。

```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello_cargo`
Hello, world!
```

これで新規プロジェクトを作成し、コードを実行するまでの流れである。

</details>

## QN

<details>
<summary>回答例</summary>
</details>

## 参考情報

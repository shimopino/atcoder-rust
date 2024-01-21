# Atcoder in Rust

`cargo-atcoder` を使用して問題を解いていく。

https://github.com/tanakh/cargo-atcoder

```bash
# 新しくコンテストに取り組む場合
$ cargo atcoder new abc152

$ cd abc152

# 提出
$ cargo atcoder submit a [--bin]

# ステータスの確認
$ cargo atcoder status

# テスト
$ cargo atcoder test a [--custom]
```

使用するクレート

```toml
proconio = { version = "0.4.3", features = ["derive"] }
```

- 2020年版
  - https://github.com/rust-lang-ja/atcoder-rust-resources/wiki/2020-Update
- 2023年版
  - https://github.com/rust-lang-ja/atcoder-rust-resources/wiki/Jan-2023-Language-Update-%E3%83%A9%E3%82%A4%E3%83%96%E3%83%A9%E3%83%AA%E6%A1%88

## Resources

- Beginner
  - [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/title-page.html)
- .gitignore
  - [Rust.gitignore](https://github.com/github/gitignore/blob/master/Rust.gitignore)

## Status

- [ABC335](./abc335/)
  - A, B, C, D

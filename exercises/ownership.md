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

## Q2

Rustにて以下のコードの実行結果はどのようになるのでしょうか。

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

<details>
<summary>回答例</summary>

`String` 型の値はスタックではなく、ヒープメモリに格納されている（ユーザー入力などの実行時にサイズが確定する変数を保存するため）。

変数にはヒープメモリに格納されている値ではなく、スタックに積まれているヒープメモリへのポインタや現在使用しているメモリ量、OSから受け取った全メモリ量などの情報が格納されている。

つまり単純に変数をほかの変数に代入すると、このスタックのデータがコピーされてしまい、新しい変数の同じヒープメモリを参照していることになる。

```rust
let s1 = String::from("hello"); // ヒープメモリにStringを作成。変数にはポインタなど
let s2 = s1;                    // スタックにあるポインタなどがコピーされる
```

![](https://doc.rust-jp.rs/book-ja/img/trpl04-02.svg)

Rustの特徴的な部分は、変数の2重解放エラーを防止するために、上記のようなコードの場合に `s1` が有効ではないと判定し確保されているメモリを解放する。

つまり変数 `s1` には参照することができなくなる。

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);     // コンパイルエラーが発生
```

</details>

## Q3

以下のコードと、問題2のコードの違いは何でしょうか。

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

<details>
<summary>回答例</summary>

問題2では変数に格納されているスタックにあるポインタなどのデータがコピーされる。

そのためRustの **ムーブ** という機能が働き、変数 `s1` が格納しているスタックのデータは解放されてアクセスすることができなくなる。

`clone()` を使用すると、ヒープメモリに存在するデータ自体をコピーするため、変数 `s1` には引き続いてアクセスすることが可能となっている。

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);   // s1にはアクセス可能である
```

</details>

## QN

<details>
<summary>回答例</summary>



</details>

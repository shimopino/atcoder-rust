# Rustの概念

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
<details>
<summary>Table of Contents</summary>

- [Q1](#q1)
- [Q2](#q2)
- [Q3](#q3)
- [Q4](#q4)
- [Q5](#q5)
- [Q6](#q6)
- [Q7](#q7)
- [Q8](#q8)
- [Q9](#q9)
- [Q10](#q10)
- [QN](#qn)

</details>
<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Q1

32ビットの非負数値で名称が `MAX_POINTS` であり、値が100000である定数を定義してみましょう。

<details>
<summary>回答例</summary>

Rustでは定数の命名規則として、全て大文字でアンダースコアで単語区切りする必要がある。

```rust
const MAX_POINTS: u32 = 100_000;
```

</details>

## Q2

以下の2つのコードの違いは何でしょうか。

コード1

```rust
let spaces = "    ";
let spaces = spaces.len();
```

コード2

```rust
let mut spaces = "    ";
spaces = spaces.len();
```

<details>
<summary>回答例</summary>

コード1では、不変の文字列型変数として `spaces` を定義している。

そのあとで文字列長を計算し、再度不変の数値型の変数 `spaces` として、新しい変数を生成している。

この方法では、同じ変数に対して異なる値を割り当てることが可能になる。

```rust
let spaces = "    ";
let spaces = spaces.len();
```

コード2では、可変の文字列型として `spaces` を定義している。

そのあとで文字列長を数値型として再代入しようとしているが、`spaces` の型は文字列型のままなので、型の不一致が発生してしまい、下記コードはコンパイルすることはできない。

```rust
let mut spaces = "    ";
spaces = spaces.len();
```

</details>

## Q3

Rustで使用できる組み込みの整数型はどのようなものがあるでしょうか。

<details>
<summary>回答例</summary>

整数はビット数と符号のあるなしで分類できる。

|大きさ|符号付き|符号なし|
|:--|:--|:--|
|8ビット|`i8`|`u8`|
|16ビット|`i16`|`u16`|
|32ビット|`i32`|`u32`|
|64ビット|`i64`|`u64`|
|アーキテクチャ依存|`isize`|`usize`|

デフォルトでは `i32` が使用される。

</details>

## Q4

Rustで使用できる組み込みの浮動小数点型はどのようなものがあるでしょうか。

<details>
<summary>回答例</summary>
</details>

- `f32`
  - 32ビットの浮動小数点。単精度浮動小数点数
- `f64`
  - 64ビットの浮動小数点。倍精度浮動小数点数

デフォルトでは `f64` が使用される。

```rust
fn main {
    // f64
    let x = 2.0;

    let y: f32 = 3.0;
}
```

## Q5

Rustで論理値型を定義するにはどうすればいいでしょうか。

<details>
<summary>回答例</summary>

Rustでは論理値型 (`bool`) として `true` と `false` を使用することができる。

```rust
fn main() {
    let t = true;

    let f: bool = false;
}
```

</details>

## Q6

Rustで `cahr` をどのように定義すればいいでしょうか

<details>
<summary>回答例</summary>

Rustでは **シングルクォート** を使用することで `char` を定義することができる。

（なお文字列はダブルクォートを使用する）

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';    //ハート目の猫
}
```

なおユニコードのスカラー値をあらわしている。

</details>

## Q7

符号付き32ビット整数、64ビットの浮動小数点、8ビットの非負整数を要素として持っているタプルを `tup` という名称の変数で宣言してみましょう。

またパターンマッチングを使用して各要素を取得する方法と、各要素に添え字を使用して直接取得する方法を試してみましょう。

<details>
<summary>回答例</summary>

```rust
fn main() {
    // タプルの宣言
    let tup: (i32, f64, u8) = (500, 0.1, 1);
    
    // パターンマッチング
    let (a, b, c) = tup;

    println!("The value of a, b, c is: {}, {}, {}", a, b, c);

    // 要素への直接アクセス
    println!("tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);
}
```

</details>

## Q8

1から5までの32ビットの符号付き整数を要素に持つ配列を定義してみましょう。

また各要素にはどうアクセスすればいいでしょうか。

<details>
<summary>回答例</summary>

Rustでは、配列は全ての要素が同じ型であり、固定長である必要がある。

```rust
let a = [1, 2, 3, 4, 5];
```

各要素には添え字でアクセスできる。

```rust
let a = [1, 2, 3, 4, 5];

let first = a[0];
let last = a[a.len() - 1];
```

</details>

## Q9

プログラムのエントリポインタ `main` から `Hello World` を出力し、この関数の中から別途に定義している `Another Function` という文字列を出力する `another_function` 関数を起動してみましょう。

<details>
<summary>回答例</summary>

```rust
fn main() {
    println!("Hello World");

    another_function();
}

fn another_function() {
    println!("Another Function");
}
```

</details>

## Q10

8ビットの非負整数の `age` を受け取り、`your age is {age}` を出力する関数を作成して、`main` から呼び出してみましょう。

<details>
<summary>回答例</summary>

```rust
fn main() {
    print_age(100);
}

fn print_age(age: u8) {
    println!("Your age is {}", age);
}
```

</details>

## Q11

以下のコードを実行した結果は何になるのでしょうか。

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

<details>
<summary>回答例</summary>

新しいスコープを作成する際に使用するブロック (`{}`) は式であり、評価値を `let` 文の一部として変数 `y` に束縛する形になる。

そのため出力結果は `The value of y is: 4` となる。

</details>

## Q12

引数に符号付き32ビット整数を与えて、引数の値に1を足したものを返す関数 `plus_one` を作成してみましょう。

<details>
<summary>回答例</summary>

注意点は関数内をセミコロンで終わってしまうと、文だと評価されてしまい、文は値には評価されないためにコンパイラが失敗する点である。

```rust
fn main() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

</details>

## Q13

Rustにて、数値型の変数 `number` に対して、ある条件が真の場合は `5` を代入し、それ以外の場合には `6` を代入するにはどうすればいいでしょうか。

イメージとしては三項演算子に近い。

<details>
<summary>回答例</summary>

```rust
let conditioon = true;
let number = if condition {
    5
} else {
    6
};
```

</details>

## Q14

Rustでは以下のようにループを使用して配列にアクセスすることができる。

```rust
let a = [10, 20, 30, 40, 50];
let mut index = 0;

while index < 5 {
    // 値は{}です
    println!("the value is: {}", a[index]);

    index = index + 1;
}
```

しかしこの方法は配列の要素数を明示的に指定する必要があるためバグが発生する可能性が存在する。

では配列のメソッドを使用して、配列の各要素にアクセスしてみましょう。

<details>
<summary>回答例</summary>

配列の `iter()` を使用する。

```rust
let a = [1, 2, 3, 4, 5];

for element in a.iter() {
    println!("The value is {}", element);
}
```

</details>

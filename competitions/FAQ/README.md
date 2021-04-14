# 競プロに関するFAQ

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
<details>
<summary>Table of Contents</summary>

- [Q1 `proconio` を使用して標準入力から空白区切りで値を取得するには](#q1-proconio-%E3%82%92%E4%BD%BF%E7%94%A8%E3%81%97%E3%81%A6%E6%A8%99%E6%BA%96%E5%85%A5%E5%8A%9B%E3%81%8B%E3%82%89%E7%A9%BA%E7%99%BD%E5%8C%BA%E5%88%87%E3%82%8A%E3%81%A7%E5%80%A4%E3%82%92%E5%8F%96%E5%BE%97%E3%81%99%E3%82%8B%E3%81%AB%E3%81%AF)
- [Q2 値が偶数か奇数かに応じて文字列を返す処理を1行で表現するには](#q2-%E5%80%A4%E3%81%8C%E5%81%B6%E6%95%B0%E3%81%8B%E5%A5%87%E6%95%B0%E3%81%8B%E3%81%AB%E5%BF%9C%E3%81%98%E3%81%A6%E6%96%87%E5%AD%97%E5%88%97%E3%82%92%E8%BF%94%E3%81%99%E5%87%A6%E7%90%86%E3%82%921%E8%A1%8C%E3%81%A7%E8%A1%A8%E7%8F%BE%E3%81%99%E3%82%8B%E3%81%AB%E3%81%AF)
- [Q3 `101` などの文字列型の値から1文字ごとに char を取り出すにはどうすればいいでしょうか](#q3-101-%E3%81%AA%E3%81%A9%E3%81%AE%E6%96%87%E5%AD%97%E5%88%97%E5%9E%8B%E3%81%AE%E5%80%A4%E3%81%8B%E3%82%891%E6%96%87%E5%AD%97%E3%81%94%E3%81%A8%E3%81%AB-char-%E3%82%92%E5%8F%96%E3%82%8A%E5%87%BA%E3%81%99%E3%81%AB%E3%81%AF%E3%81%A9%E3%81%86%E3%81%99%E3%82%8C%E3%81%B0%E3%81%84%E3%81%84%E3%81%A7%E3%81%97%E3%82%87%E3%81%86%E3%81%8B)
- [Q4 整数a,bが与えられた際に倍数判定を行う式は何でしょうか](#q4-%E6%95%B4%E6%95%B0ab%E3%81%8C%E4%B8%8E%E3%81%88%E3%82%89%E3%82%8C%E3%81%9F%E9%9A%9B%E3%81%AB%E5%80%8D%E6%95%B0%E5%88%A4%E5%AE%9A%E3%82%92%E8%A1%8C%E3%81%86%E5%BC%8F%E3%81%AF%E4%BD%95%E3%81%A7%E3%81%97%E3%82%87%E3%81%86%E3%81%8B)
- [Q5 u32の要素をもつイテレータに対して filter とfilter_map を適用する場合の違いは何か](#q5-u32%E3%81%AE%E8%A6%81%E7%B4%A0%E3%82%92%E3%82%82%E3%81%A4%E3%82%A4%E3%83%86%E3%83%AC%E3%83%BC%E3%82%BF%E3%81%AB%E5%AF%BE%E3%81%97%E3%81%A6-filter-%E3%81%A8filter_map-%E3%82%92%E9%81%A9%E7%94%A8%E3%81%99%E3%82%8B%E5%A0%B4%E5%90%88%E3%81%AE%E9%81%95%E3%81%84%E3%81%AF%E4%BD%95%E3%81%8B)
- [Q6 イテレータに対して collect を適用すると何が返されるのか](#q6-%E3%82%A4%E3%83%86%E3%83%AC%E3%83%BC%E3%82%BF%E3%81%AB%E5%AF%BE%E3%81%97%E3%81%A6-collect-%E3%82%92%E9%81%A9%E7%94%A8%E3%81%99%E3%82%8B%E3%81%A8%E4%BD%95%E3%81%8C%E8%BF%94%E3%81%95%E3%82%8C%E3%82%8B%E3%81%AE%E3%81%8B)
- [Q7 イテレータに対して map を適用すると何が返されるのか](#q7-%E3%82%A4%E3%83%86%E3%83%AC%E3%83%BC%E3%82%BF%E3%81%AB%E5%AF%BE%E3%81%97%E3%81%A6-map-%E3%82%92%E9%81%A9%E7%94%A8%E3%81%99%E3%82%8B%E3%81%A8%E4%BD%95%E3%81%8C%E8%BF%94%E3%81%95%E3%82%8C%E3%82%8B%E3%81%AE%E3%81%8B)
- [Q8 イテレータの要素にアクセスする場合の `|x|` と `|&x|` の違いは何か](#q8-%E3%82%A4%E3%83%86%E3%83%AC%E3%83%BC%E3%82%BF%E3%81%AE%E8%A6%81%E7%B4%A0%E3%81%AB%E3%82%A2%E3%82%AF%E3%82%BB%E3%82%B9%E3%81%99%E3%82%8B%E5%A0%B4%E5%90%88%E3%81%AE-x-%E3%81%A8-x-%E3%81%AE%E9%81%95%E3%81%84%E3%81%AF%E4%BD%95%E3%81%8B)
- [Q9 u32 の要素を持つコレクションから、2進数表記の場合の末尾の0を数え上げてその最小値を求める場合にどうすればいいでしょうか](#q9-u32-%E3%81%AE%E8%A6%81%E7%B4%A0%E3%82%92%E6%8C%81%E3%81%A4%E3%82%B3%E3%83%AC%E3%82%AF%E3%82%B7%E3%83%A7%E3%83%B3%E3%81%8B%E3%82%892%E9%80%B2%E6%95%B0%E8%A1%A8%E8%A8%98%E3%81%AE%E5%A0%B4%E5%90%88%E3%81%AE%E6%9C%AB%E5%B0%BE%E3%81%AE0%E3%82%92%E6%95%B0%E3%81%88%E4%B8%8A%E3%81%92%E3%81%A6%E3%81%9D%E3%81%AE%E6%9C%80%E5%B0%8F%E5%80%A4%E3%82%92%E6%B1%82%E3%82%81%E3%82%8B%E5%A0%B4%E5%90%88%E3%81%AB%E3%81%A9%E3%81%86%E3%81%99%E3%82%8C%E3%81%B0%E3%81%84%E3%81%84%E3%81%A7%E3%81%97%E3%82%87%E3%81%86%E3%81%8B)
- [Q10 10進数表記されている数値の、各桁の合計値を計算するにはどうすればいいでしょうか](#q10-10%E9%80%B2%E6%95%B0%E8%A1%A8%E8%A8%98%E3%81%95%E3%82%8C%E3%81%A6%E3%81%84%E3%82%8B%E6%95%B0%E5%80%A4%E3%81%AE%E5%90%84%E6%A1%81%E3%81%AE%E5%90%88%E8%A8%88%E5%80%A4%E3%82%92%E8%A8%88%E7%AE%97%E3%81%99%E3%82%8B%E3%81%AB%E3%81%AF%E3%81%A9%E3%81%86%E3%81%99%E3%82%8C%E3%81%B0%E3%81%84%E3%81%84%E3%81%A7%E3%81%97%E3%82%87%E3%81%86%E3%81%8B)
- [Q11 Vec型の配列からイテレータを作成して逆順でアクセスするにはどうすればいいでしょうか](#q11-vec%E5%9E%8B%E3%81%AE%E9%85%8D%E5%88%97%E3%81%8B%E3%82%89%E3%82%A4%E3%83%86%E3%83%AC%E3%83%BC%E3%82%BF%E3%82%92%E4%BD%9C%E6%88%90%E3%81%97%E3%81%A6%E9%80%86%E9%A0%86%E3%81%A7%E3%82%A2%E3%82%AF%E3%82%BB%E3%82%B9%E3%81%99%E3%82%8B%E3%81%AB%E3%81%AF%E3%81%A9%E3%81%86%E3%81%99%E3%82%8C%E3%81%B0%E3%81%84%E3%81%84%E3%81%A7%E3%81%97%E3%82%87%E3%81%86%E3%81%8B)
- [Q12 イテレータに対して `enumerate` を使用するとどのような値が返ってくるでしょうか](#q12-%E3%82%A4%E3%83%86%E3%83%AC%E3%83%BC%E3%82%BF%E3%81%AB%E5%AF%BE%E3%81%97%E3%81%A6-enumerate-%E3%82%92%E4%BD%BF%E7%94%A8%E3%81%99%E3%82%8B%E3%81%A8%E3%81%A9%E3%81%AE%E3%82%88%E3%81%86%E3%81%AA%E5%80%A4%E3%81%8C%E8%BF%94%E3%81%A3%E3%81%A6%E3%81%8F%E3%82%8B%E3%81%A7%E3%81%97%E3%82%87%E3%81%86%E3%81%8B)
- [Q13 イテレータに対して `zip` を `enumerate` と同じように処理させるにはどうすればいいでしょうか](#q13-%E3%82%A4%E3%83%86%E3%83%AC%E3%83%BC%E3%82%BF%E3%81%AB%E5%AF%BE%E3%81%97%E3%81%A6-zip-%E3%82%92-enumerate-%E3%81%A8%E5%90%8C%E3%81%98%E3%82%88%E3%81%86%E3%81%AB%E5%87%A6%E7%90%86%E3%81%95%E3%81%9B%E3%82%8B%E3%81%AB%E3%81%AF%E3%81%A9%E3%81%86%E3%81%99%E3%82%8C%E3%81%B0%E3%81%84%E3%81%84%E3%81%A7%E3%81%97%E3%82%87%E3%81%86%E3%81%8B)
- [Q14 Vec型から重複した要素を削除するにはどうすればいいでしょうか](#q14-vec%E5%9E%8B%E3%81%8B%E3%82%89%E9%87%8D%E8%A4%87%E3%81%97%E3%81%9F%E8%A6%81%E7%B4%A0%E3%82%92%E5%89%8A%E9%99%A4%E3%81%99%E3%82%8B%E3%81%AB%E3%81%AF%E3%81%A9%E3%81%86%E3%81%99%E3%82%8C%E3%81%B0%E3%81%84%E3%81%84%E3%81%A7%E3%81%97%E3%82%87%E3%81%86%E3%81%8B)
- [Q15 Stringで初期化した文字列や、ダブルクォーテーションで指定した文字列が空文字であることを確認するには](#q15-string%E3%81%A7%E5%88%9D%E6%9C%9F%E5%8C%96%E3%81%97%E3%81%9F%E6%96%87%E5%AD%97%E5%88%97%E3%82%84%E3%83%80%E3%83%96%E3%83%AB%E3%82%AF%E3%82%A9%E3%83%BC%E3%83%86%E3%83%BC%E3%82%B7%E3%83%A7%E3%83%B3%E3%81%A7%E6%8C%87%E5%AE%9A%E3%81%97%E3%81%9F%E6%96%87%E5%AD%97%E5%88%97%E3%81%8C%E7%A9%BA%E6%96%87%E5%AD%97%E3%81%A7%E3%81%82%E3%82%8B%E3%81%93%E3%81%A8%E3%82%92%E7%A2%BA%E8%AA%8D%E3%81%99%E3%82%8B%E3%81%AB%E3%81%AF)
- [Q16 String型に対して末尾が所定の文字列であることを確認するには](#q16-string%E5%9E%8B%E3%81%AB%E5%AF%BE%E3%81%97%E3%81%A6%E6%9C%AB%E5%B0%BE%E3%81%8C%E6%89%80%E5%AE%9A%E3%81%AE%E6%96%87%E5%AD%97%E5%88%97%E3%81%A7%E3%81%82%E3%82%8B%E3%81%93%E3%81%A8%E3%82%92%E7%A2%BA%E8%AA%8D%E3%81%99%E3%82%8B%E3%81%AB%E3%81%AF)
- [Q17 String型の文字列から所定の長さの文字を抽出したい場合は](#q17-string%E5%9E%8B%E3%81%AE%E6%96%87%E5%AD%97%E5%88%97%E3%81%8B%E3%82%89%E6%89%80%E5%AE%9A%E3%81%AE%E9%95%B7%E3%81%95%E3%81%AE%E6%96%87%E5%AD%97%E3%82%92%E6%8A%BD%E5%87%BA%E3%81%97%E3%81%9F%E3%81%84%E5%A0%B4%E5%90%88%E3%81%AF)
- [QN](#qn)
- [QN](#qn-1)

</details>
<!-- END doctoc generated TOC please keep comment here to allow auto update -->

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

## Q3 `101` などの文字列型の値から1文字ごとに char を取り出すにはどうすればいいでしょうか

<details>
<summary>回答</summary>

`String` が提供する `cahrs()` ではイテレータ `Iterator<Item = char>` を返す。

後はイテレータに対して繰り返し処理を実装すればいい。

複数行の場合は以下になる

```rust
input! {
    s: String
};
let mut result: i32 = 0;
// イテレータから各要素を取得して繰り返す
for ichar in s.chars() {
    if ichar == '1' {
        result += 1;
    }
}
```

クロージャを使用する場合は以下になる。

```rust
    input! {
        s: String
    };
    let result = s.chars()                  // イテレータを取得
                  .filter(|&c| c == '1')    // 各要素を取り出してクロージャを適用
                  .count();                 // 要素数を数える
```

</details>

## Q4 整数a,bが与えられた際に倍数判定を行う式は何でしょうか

<details>
<summary>回答</summary>

```math
(a + b - 1) / b
```

</details>

## Q5 u32の要素をもつイテレータに対して filter とfilter_map を適用する場合の違いは何か

<details>
<summary>回答</summary>

`filter` では引数で与えられた関数が `true` になる要素のみを返すような、新たなイテレータを返す。

注目する点は、あくまでも各要素を不変な参照でアクセスしているため、イテレータの中身は一切 **変更せず** にただ単に要素の選択にのみ使用できる点である。

```rust
filter(): Iterator<T> -> (&T -> bool) -> Iterator<T>
```

`filter_map` では引数で各要素を受け取り、`Option` を返す関数を適用することで、`Some` だったものだけを新たな要素のイテレータとして返す。

これは各要素の値を受け取って、要素自体に変更を加える場合に使用できる。

```rust
filter_map(): Iterator<T> -> (T -> Option<U>) -> Iterator<U>
```

例えば [公式ドキュメント](https://doc.rust-lang.org/1.41.0/std/iter/trait.Iterator.html#examples-11) から以下のような使用例がある。

```rust
let a = ["1", "lol", "3", "NaN", "5"];

let mut iter = a.iter().filter_map(|s| s.parse().ok());

assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(3));
assert_eq!(iter.next(), Some(5));
assert_eq!(iter.next(), None);
```

</details>

## Q6 イテレータに対して collect を適用すると何が返されるのか

<details>
<summary>回答</summary>

`collect` では、イテレータの全要素を取り出して、新たなコレクションを作成できる。

```rust
collect(): Iterator<T> -> Collection of T
```

[公式ドキュメント](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) にある通り、ほかのメソッドを使用して作成した新たなイテレータからコレクションを作成することができる。

```rust
let a = [1, 2, 3];

let doubled: Vec<i32> = a.iter()            // コレクションからイテレータを作成
                         .map(|&x| x * 2)   // 各要素を2倍する
                         .collect();        // イテレータからコレクションを作成

assert_eq!(vec![2, 4, 6], doubled);
```

</details>

## Q7 イテレータに対して map を適用すると何が返されるのか

<details>
<summary>回答</summary>

`map` では各要素に対して何かしら変更を加えた新たなイテレータを返す。

```rust
map(): Iterator<T> -> (T -> U) -> Iterator<U>
```

以下の [公式ドキュメント](https://doc.rust-lang.org/1.41.0/std/iter/trait.Iterator.html#method.map) の例がわかりやすい。

```rust
let a = [1, 2, 3];

let mut iter = a.iter().map(|x| 2 * x);

assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(4));
assert_eq!(iter.next(), Some(6));
assert_eq!(iter.next(), None);
```

もしも `map` した要素に対して条件をもとに要素をフィルタリングしたい場合は `filter_map` を使用する。

</details>

## Q8 イテレータの要素にアクセスする場合の `|x|` と `|&x|` の違いは何か

<details>
<summary>回答</summary>


</details>

## Q9 u32 の要素を持つコレクションから、2進数表記の場合の末尾の0を数え上げてその最小値を求める場合にどうすればいいでしょうか

<details>
<summary>回答</summary>

整数に対してビット演算を行う関数が標準で提供されている。

```rust
n.count_ones();     // ビット表現したときに現れる1の数を求める
n.count_zeros();    // ビットで表現したときに現れる0の数を求める
n.leading_zeros();  // ビットで表現したときの頭の0の数を求める
n.trailing_zeros(); // ビットで表現したときの末尾の0の数を求める
n.swap_bytes();     // byte順序を逆にする
n.rotate_right(4);  // ラップする右シフト 
n.rotate_left(4);   // ラップする左シフト
```

今回ではこの中から `trailing_zeros` を使用すればいい。

配列から2進数表記の末尾が0の数の最小値は以下のように求めることが可能である。

```rust
let result = {
    a.iter()                        // イテレータの作成
     .map(|&x| x.trailing_zeros())  // 末尾の0の数を計算
     .min()                         // 最小値を Option<T> で返す
     .unwrap()                      // Optionをはがす
};
```

ポイントは `min()` が返す値が、イテレータの要素数が0の場合も考慮して `Option<T>` を返す設計となっている点である。

</details>

## Q10 10進数表記されている数値の、各桁の合計値を計算するにはどうすればいいでしょうか

<details>
<summary>回答</summary>

変数に `123` が格納されており、各桁の合計値である `6` を計算することを考える。 

```rust
let sum = i.to_string()                 // まずは数値型を文字列型に変換する
            .chars()                    // 各桁を取得するために char のイテレータを作成
            .map(|c| {                  // char の各要素にアクセス
                c.to_digit(10).unwrap() // 10進数表記に変換する。その際にOptionは取り外す
            })
            .sum();                     // Integer の要素の合計値を計算する
```

注意点は `to_digit` で変換する場合には `Option<T>` が返されるため `unwrap()` で剝がしておく必要がある点である。

</details>

## Q11 Vec型の配列からイテレータを作成して逆順でアクセスするにはどうすればいいでしょうか

<details>
<summary>回答</summary>

`iter()` でイテレータを作成して、イテレータの `rev` メソッドを使用すれば、順番を逆にして返すような新しいイテレータを返す。

```rust
rev(): DoubleEndedIterator<T> -> DoubleEndedIterator<T>
```

これは要素の右から左へ繰り返すようにするため、必ず End が存在するイテレータである必要があり、そのために `DoubleEndedIterator` に依存している。

[公式ドキュメント](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rev) の事例がわかりやすい。

```rust
let a = [1, 2, 3];

let mut iter = a.iter().rev();

assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), Some(&1));

assert_eq!(iter.next(), None);
```

</details>

## Q12 イテレータに対して `enumerate` を使用するとどのような値が返ってくるでしょうか

<details>
<summary>回答</summary>

`enumerate` を使用すると、呼び出した時点を最初として、要素の順番を `(順番インデックス, 要素)` のタプルで返すようなイテレータを作成する。

```rust
enumerate(): Iterator<T> -> Iterator<(usize, T)>
```

[公式ドキュメント](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate) にある通り、あくまでの順番のインデックスを `usize` で返すため、ほかの型を使用したい場合には `zip` を代わりに使用したほうがよさそう。

```rust
let a = ['a', 'b', 'c'];

let mut iter = a.iter().enumerate();

assert_eq!(iter.next(), Some((0, &'a')));
assert_eq!(iter.next(), Some((1, &'b')));
assert_eq!(iter.next(), Some((2, &'c')));
assert_eq!(iter.next(), None);
```

</details>

## Q13 イテレータに対して `zip` を `enumerate` と同じように処理させるにはどうすればいいでしょうか

<details>
<summary>回答</summary>

`zip` では2つのイテレータから両方の要素を順番にアクセスしていき、そのペアを返すようなイテレータを作成する。

```rust
zip(): Iterator<T> -> IntoIterator<U> -> Iterator<T, U>
```

[公式ドキュメント](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip) にもある通り、以下のようにイテレータを指定することで、`enumerate` と同じような動作をさせることができる。

```rust
let enumerate: Vec<_> = "foo".chars().enumerate().collect();

let zipper: Vec<_> = (0..).zip("foo".chars()).collect();

assert_eq!((0, 'f'), enumerate[0]);
assert_eq!((0, 'f'), zipper[0]);

assert_eq!((1, 'o'), enumerate[1]);
assert_eq!((1, 'o'), zipper[1]);

assert_eq!((2, 'o'), enumerate[2]);
assert_eq!((2, 'o'), zipper[2]);
```

</details>

## Q14 Vec型から重複した要素を削除するにはどうすればいいでしょうか

<details>
<summary>回答</summary>

`dedup` を使用することで、連続して重複している要素を削除することができる。

これは [公式ドキュメント](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup) の例がわかりやすい

```rust
let mut vec = vec![1, 2, 2, 3, 2];

dev.dedup();

assert_eq!(vec, [1, 2, 3, 2]);
```

全ての重複する要素を削除したい場合は事前にソートをしておくことで削除できる。

</details>

## Q15 Stringで初期化した文字列や、ダブルクォーテーションで指定した文字列が空文字であることを確認するには

<details>
<summary>回答</summary>

`is_empty` を使用することで空文字判定に使用できる。

[Primitiveな文字列の場合](https://doc.rust-lang.org/stable/std/primitive.str.html#method.is_empty) は以下のように、0バイトであるかどうかを判定している。

```rust
let s = "";
assert!(s.is_empty());

let s = "not empty";
assert!(!s.is_empty());
```

[`String` 型の場合](https://doc.rust-lang.org/std/string/struct.String.html#method.is_empty) には以下のように長さが0であることを確認している。

```rust
let mut v = String::new();
assert!(v.is_empty());

v.push('a');
assert!(!v.is_empty());
```

</details>

## Q16 String型に対して末尾が所定の文字列であることを確認するには

<details>
<summary>回答</summary>

[`ends_with`](https://doc.rust-lang.org/std/string/struct.String.html#method.ends_with) を使用して、末尾が指定したパターンにマッチしているのか判定できる。

```rust
let bananas = "bananas";

assert!(bananas.ends_with("anas"));
assert!(!bananas.ends_with("nana"));
```

</details>

## Q17 String型の文字列から所定の長さの文字を抽出したい場合は

<details>
<summary>回答</summary>

[`truncate`](https://doc.rust-lang.org/std/string/struct.String.html#method.truncate) を使用すればいい。

```rust
let mut s = String::from("hello");

s.truncate(2);

assert_eq!("he", s);
```

</details>

## QN

<details>
<summary>回答</summary>


</details>

## QN

<details>
<summary>回答</summary>


</details>

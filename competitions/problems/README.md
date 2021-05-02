# 問題集

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
<details>
<summary>Table of Contents</summary>

- [全探索](#%E5%85%A8%E6%8E%A2%E7%B4%A2)
- [再帰と分割統治法](#%E5%86%8D%E5%B8%B0%E3%81%A8%E5%88%86%E5%89%B2%E7%B5%B1%E6%B2%BB%E6%B3%95)
- [動的計画法](#%E5%8B%95%E7%9A%84%E8%A8%88%E7%94%BB%E6%B3%95)
- [二分探索法](#%E4%BA%8C%E5%88%86%E6%8E%A2%E7%B4%A2%E6%B3%95)
- [貪欲法](#%E8%B2%AA%E6%AC%B2%E6%B3%95)
- [配列、連結リスト、ハッシュテーブル](#%E9%85%8D%E5%88%97%E9%80%A3%E7%B5%90%E3%83%AA%E3%82%B9%E3%83%88%E3%83%8F%E3%83%83%E3%82%B7%E3%83%A5%E3%83%86%E3%83%BC%E3%83%96%E3%83%AB)
- [スタック、キュー](#%E3%82%B9%E3%82%BF%E3%83%83%E3%82%AF%E3%82%AD%E3%83%A5%E3%83%BC)
- [グラフと木](#%E3%82%B0%E3%83%A9%E3%83%95%E3%81%A8%E6%9C%A8)
- [Union-Find](#union-find)
- [グラフ探索](#%E3%82%B0%E3%83%A9%E3%83%95%E6%8E%A2%E7%B4%A2)
- [最短路](#%E6%9C%80%E7%9F%AD%E8%B7%AF)
- [最小全域木](#%E6%9C%80%E5%B0%8F%E5%85%A8%E5%9F%9F%E6%9C%A8)
- [ネットワークフロー](#%E3%83%8D%E3%83%83%E3%83%88%E3%83%AF%E3%83%BC%E3%82%AF%E3%83%95%E3%83%AD%E3%83%BC)
- [参考資料](#%E5%8F%82%E8%80%83%E8%B3%87%E6%96%99)

</details>
<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## 準備問題

- 倍数判定
  - [ABC 086 A - Product](https://atcoder.jp/contests/abc086/tasks/abc086_a)
  - [ABC 064 A - RGB Cards](https://atcoder.jp/contests/abc064/tasks/abc064_a)
  - [ABC 088 A - Infinite Coins](https://atcoder.jp/contests/abc088/tasks/abc088_a)
  - [ABC 157 A - Duplex Printing](https://atcoder.jp/contests/abc157/tasks/abc157_a)
- 文字列操作
  - [ABC 081 A - Placing Marbles](https://atcoder.jp/contests/abc081/tasks/abc081_a)
  - [ABC 095 A - Something on It](https://atcoder.jp/contests/abc095/tasks/abc095_a)
  - [ABC 085 A - Already 2018](https://atcoder.jp/contests/abc085/tasks/abc085_a)
  - [ABC 069 B - i18n](https://atcoder.jp/contests/abc069/tasks/abc069_b)
  - [ABC 082 B - Two Anagrams](https://atcoder.jp/contests/abc082/tasks/abc082_b)
- N進数
  - [ABC 083 B - Some Sums](https://atcoder.jp/contests/abc083/tasks/abc083_b)
  - [ABC 090 B - Palindromic Numbers](https://atcoder.jp/contests/abc090/tasks/abc090_b)
  - [AGC 025 A - Digits Sum](https://atcoder.jp/contests/agc025/tasks/agc025_a)
  - [ABC 156 B - Digits](https://atcoder.jp/contests/abc156/tasks/abc156_b)
- ソート
  - [ABC 088 B - Card Game for Two](https://atcoder.jp/contests/abc088/tasks/abc088_b)
  - [ABC 067 B - Snake Toy](https://atcoder.jp/contests/abc067/tasks/abc067_b)
  - [ABC 042 B - Iroha Loves Strings](https://atcoder.jp/contests/abc042/tasks/abc042_b)
  - [AGC 027 A - Candy Distribution Again](https://atcoder.jp/contests/agc027/tasks/agc027_a)
  - [AGC 012 A - AtCoder Group Contest](https://atcoder.jp/contests/agc012/tasks/agc012_a)
- 連想配列
  - [ABC 085 B - Kagami Mochi](https://atcoder.jp/contests/abc085/tasks/abc085_b)
  - [ABC 071 B - Not Found](https://atcoder.jp/contests/abc071/tasks/abc071_b)
  - [ABC 061 B - Counting Roads](https://atcoder.jp/contests/abc061/tasks/abc061_b)
  - [ABC 047 B - Snuke's Coloring 2 (ABC Edit)](https://atcoder.jp/contests/abc047/tasks/abc047_b)
  - [ABC 091 B Two Colors Card Game](https://atcoder.jp/contests/abc091/tasks/abc091_b)
  - [ABC 081 C - Not so Diverse](https://atcoder.jp/contests/abc081/tasks/arc086_a)
- パリティ
  - [ABC 086 C - Traveling](https://atcoder.jp/contests/abc086/tasks/arc089_a)
  - [ABC 093 C - Same Integers](https://atcoder.jp/contests/abc093/tasks/arc094_a)
  - [AGC 010 A - Addition](https://atcoder.jp/contests/agc010/tasks/agc010_a)
  - [AGC 020 A - Move and Win](https://atcoder.jp/contests/agc020/tasks/agc020_a)
  - [ABC 073 C - Write and Erase](https://atcoder.jp/contests/abc073/tasks/abc073_c)
- グリッド
  - [ABC 075 B](https://atcoder.jp/contests/abc075/tasks/abc075_b)
  - [ABC 096 C](https://atcoder.jp/contests/abc096/tasks/abc096_c)
- 区間重なり
  - [ABC 070 B](https://atcoder.jp/contests/abc070/tasks/abc070_b)
- 余り
  - [ABC 055 B](https://atcoder.jp/contests/abc055/tasks/abc055_b)
- 数学的問題
  - [ABC 046 B](https://atcoder.jp/contests/abc046/tasks/abc046_b)
  - [ABC 048 B](https://atcoder.jp/contests/abc048/tasks/abc048_b)
- 状態ループ
  - [ABC 060 B](https://atcoder.jp/contests/abc060/tasks/abc060_b)
  - [ABC 065 B](https://atcoder.jp/contests/abc065/tasks/abc065_b)
- 累積和
  - [ABC 087 C](https://atcoder.jp/contests/abc087/tasks/arc090_a)
  - [ABC 098 C](https://atcoder.jp/contests/abc098/tasks/arc098_a)

## 全探索

- [たのしい探索アルゴリズムの世界【前編：全探索、bit全探索から半分全列挙まで】](https://qiita.com/e869120/items/25cb52ba47be0fd418d6#2-%E3%81%99%E3%81%B9%E3%81%A6%E3%81%AE%E5%9F%BA%E6%9C%AC%E5%85%A8%E6%8E%A2%E7%B4%A2)
- [ビット演算 (bit 演算) の使い方を総特集！ 〜 マスクビットから bit DP まで 〜](https://qiita.com/drken/items/7c6ff2aa4d8fce1c9361#6-bit-%E5%85%A8%E6%8E%A2%E7%B4%A2)

問題集

- 全探索
  - [ABC 081 B - Shift Only](https://atcoder.jp/contests/abc081/tasks/abc081_b)
  - [ABC 102 B - Maximum Difference](https://atcoder.jp/contests/abc102/tasks/abc102_b)
  - [ABC 113 B - Palace](https://atcoder.jp/contests/abc113/tasks/abc113_b)
  - [ABC 072 B - OddString](https://atcoder.jp/contests/abc072/tasks/abc072_b)
  - [ABC 053 B - A to Z String](https://atcoder.jp/contests/abc053/tasks/abc053_b)
  - [ABC 095 B - Bitter Alchemy](https://atcoder.jp/contests/abc095/tasks/abc095_b)
  - [ABC 133 B - Good Distance](https://atcoder.jp/contests/abc133/tasks/abc133_b)
  - [ABC 175 B - Making Triangle](https://atcoder.jp/contests/abc175/tasks/abc175_b)
  - [ABC 144 B - 81](https://atcoder.jp/contests/abc144/tasks/abc144_b)
  - [ABC 150 B - Count ABC](https://atcoder.jp/contests/abc150/tasks/abc150_b)
  - [ABC 122 B - ATCoder](https://atcoder.jp/contests/abc122/tasks/abc122_b)
  - [ABC 136 B - Uneven Numbers](https://atcoder.jp/contests/abc136/tasks/abc136_b)
  - [ABC 106 B - 105](https://atcoder.jp/contests/abc106/tasks/abc106_b)
  - [ABC 120 B - K-th Common Divisors](https://atcoder.jp/contests/abc120/tasks/abc120_b)
  - [ABC 105 B - Cakes and Donuts](https://atcoder.jp/contests/abc105/tasks/abc105_b)
  - [ABC 051 B - Sum of Three Integer](https://atcoder.jp/contests/abc051/tasks/abc051_b)
  - [ABC パ研杯2019 C - カラオケ](https://atcoder.jp/contests/pakencamp-2019-day3/tasks/pakencamp_2019_day3_c)
  - [ABC 068 B - Break Numbers](https://atcoder.jp/contests/abc068/tasks/abc068_b)
  - [ABC 157 C - Guess the Number](https://atcoder.jp/contests/abc157/tasks/abc157_c)
  - [ARC 004 A 2点間距離の最大値](https://atcoder.jp/contests/arc004/tasks/arc004_1)
  - [M-SOLUTIONSプロコンオープン2020 B - Magic 2](https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_b)
  - [三井住友信託銀行プログラミングコンテスト2019 B - Tax Rate](https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_b)
- 複雑な探索
  - [ABC 085 C - Otoshidama](https://atcoder.jp/contests/abc085/tasks/abc085_c)
  - [ABC 112 C - Pyramid](https://atcoder.jp/contests/abc112/tasks/abc112_c)
  - [ABC 088 C - Takahashi's Information](https://atcoder.jp/contests/abc088/tasks/abc088_c)
  - [ABC 057 C - Digits in Multiplication](https://atcoder.jp/contests/abc057/tasks/abc057_c)
  - [ABC 095 C - Half and Half](https://atcoder.jp/contests/abc095/tasks/arc096_a)
  - [ABC 089 C - March](https://atcoder.jp/contests/abc089/tasks/abc089_c)
  - [三井住友信託銀行プログラミングコンテスト2019 C - 100 to 105](https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_c)
  - [三井住友信託銀行プログラミングコンテスト2019 D - Lucky PIN](https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_d)
  - [JOI 2007 本選 3 - 最古の遺跡](https://atcoder.jp/contes[ts/joi2007ho/tasks/joi2007ho_c)
  - [Square869120Contest #6 B - AtCoder Markets](https://atcoder.jp/contests/s8pc-6/tasks/s8pc_6_b)
  - [JOI 2008 予選 4 - 星座探し](https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_d)
- bit全探索
  - [ABC 079 C - Train Ticket](https://atcoder.jp/contests/abc079/tasks/abc079_c)
  - [ABC 045 C - たくさんの数式](https://atcoder.jp/contests/arc061/tasks/arc061_a)
  - [ABC 104 C - All Green](https://atcoder.jp/contests/abc104/tasks/abc104_c)
  - [ARC 029 A - 高橋君とお肉](https://atcoder.jp/contests/arc029/tasks/arc029_1)
  - [ABC 002 D - 派閥](https://atcoder.jp/contests/abc002/tasks/abc002_4)
  - [ABC 128 C - Switches](https://atcoder.jp/contests/abc128/tasks/abc128_c)
  - [ABC 147 C - HonestOrUnkind2](https://atcoder.jp/contests/abc147/tasks/abc147_c)
  - [ABC 167 C - Skill Up](https://atcoder.jp/contests/abc167/tasks/abc167_c)
  - [ABC 197 C - ORXOR](https://atcoder.jp/contests/abc197/tasks/abc197_c)
  - [ABC182 C - To 3](https://atcoder.jp/contests/abc182/tasks/abc182_c)
  - [ABC190 C - Bowls and Dishes](https://atcoder.jp/contests/abc190/tasks/abc190_c)
  - [JOI 2008 予選 5 - おせんべい](https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_e)
  - [Square869120Contest #4 B - Buildings are Colorful!](https://atcoder.jp/contests/s8pc-4/tasks/s8pc_4_b)
- bit全探索 - 拡張
  - [ABC 031 D - 語呂合わせ](https://atcoder.jp/contests/abc031/tasks/abc031_d)
  - [M-SOLUTIONS プロコンオープン 2020 E - M's Solution](https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_e)
- 順列全探索
  - [ABC 145 C - Average Length](https://atcoder.jp/contests/abc145/tasks/abc145_c)
  - [ABC 150 C - Count Order](https://atcoder.jp/contests/abc150/tasks/abc150_c)
  - [ABC 054 C - One-stroke Path](https://atcoder.jp/contests/abc054/tasks/abc054_c)
  - [ABC 183 C - Travel](https://atcoder.jp/contests/abc183/tasks/abc183_c)
  - [JOI 2009 予選 D カード並べ](https://atcoder.jp/contests/joi2010yo/tasks/joi2010yo_d)
- 再帰
  - [ABC 165 C - Many Requirements](https://atcoder.jp/contests/abc165/tasks/abc165_c)
  - [CPSCO2019 Day1 C - Coins](https://atcoder.jp/contests/cpsco2019-s1/tasks/cpsco2019_s1_c)
  - [パナソニックプログラミングコンテスト 2020 D - String Equivalence](https://atcoder.jp/contests/panasonic2020/tasks/panasonic2020_d)

バチャ

- [全探索](https://kenkoooo.com/atcoder/#/contest/show/05dd88b8-9946-4356-900e-0772486976b4)

## 再帰と分割統治法

- [再帰関数を学ぶと、どんな世界が広がるか](https://qiita.com/drken/items/23a4f604fa3f505dd5ad)

問題集

- [ABC 079 B – Lucas Number](https://math.nakaken88.com/textbook/cp-fibonacci-sequence-and-recursive-function/)
- [ABC 114 C - 755](https://atcoder.jp/contests/abc114/tasks/abc114_c)
- [EDPC A – Frog 1](https://atcoder.jp/contests/dp/tasks/dp_a)
- [ABC 196 D - Hanjo](https://atcoder.jp/contests/abc196/tasks/abc196_d)

## 動的計画法

## 二分探索法

参考資料

- [競技プログラミングにおける二分探索・三分探索問題まとめ](http://hamayanhamayan.hatenablog.jp/entry/2017/07/05/160236)

問題集



## 貪欲法

## 配列、連結リスト、ハッシュテーブル

## スタック、キュー

## グラフと木

## Union-Find

## グラフ探索

## 最短路

## 最小全域木

## ネットワークフロー

## 参考資料

- [AtCoder Beginners Selection](https://atcoder.jp/contests/abs)
- [競プロ典型 90 問](https://atcoder.jp/contests/typical90)
- [AtCoder に登録したら次にやること ～ これだけ解けば十分闘える！過去問精選 10 問 ～](https://qiita.com/drken/items/fd4e5e3630d0f5859067)
- [AtCoder 版！蟻本 (初級編)](https://qiita.com/drken/items/e77685614f3c6bf86f44)
- [レッドコーダーが教える、競プロ・AtCoder上達のガイドライン【初級編：競プロを始めよう】](https://qiita.com/e869120/items/f1c6f98364d1443148b3#1-6-%E8%8C%B6%E8%89%B2%E3%82%B3%E3%83%BC%E3%83%80%E3%83%BC%E3%81%AB%E3%81%AA%E3%82%8B%E3%81%9F%E3%82%81%E3%81%AE%E3%82%AC%E3%82%A4%E3%83%89%E3%83%A9%E3%82%A4%E3%83%B3)
- [レッドコーダーが教える、競プロ・AtCoder上達のガイドライン【中級編：目指せ水色コーダー！】](https://qiita.com/e869120/items/eb50fdaece12be418faa#2-2-%E6%B0%B4%E8%89%B2%E3%82%B3%E3%83%BC%E3%83%80%E3%83%BC%E3%81%AB%E3%81%AA%E3%82%8B%E3%81%9F%E3%82%81%E3%81%AE%E3%82%AC%E3%82%A4%E3%83%89%E3%83%A9%E3%82%A4%E3%83%B3)
- [https://github.com/drken1215/book_algorithm_solution](https://github.com/drken1215/book_algorithm_solution)

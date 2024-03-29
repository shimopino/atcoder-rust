fn main() {
    // 2の補数での表現
    // 255 - 「 11111111」
    //   1 - 「 00000001」
    // 266 - 「100000000」
    //     - 「011111111」（ビット反転）
    //     - 「100000000」（1を加算）
    //     - 「 00000000」（最終結果として「0」を得る）
    assert_eq!(0, 255_u8.wrapping_add(1)); // Releaseビルドでの「255 + 1」に等しい

    // (  a +   b) mod 2 ^ N
    // (248 + 252) mod 2 ^ 8 => 244
    assert_eq!(244, 248_u8.wrapping_add(252_u8));

    // このメソッドを利用すれば、 usize でも引き算を計算できる
    // (2 + usize::MAX) mod 2^usize => 1
    assert_eq!(1_usize, 2_usize.wrapping_add(!0));

    println!("!0_i8 => {}", !0_i8); // -1
}

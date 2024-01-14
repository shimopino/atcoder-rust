use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // 2進数表記した時の末尾の0の数を数える専用の関数
    // ctz(n)
    println!("{}", n.trailing_zeros());
}

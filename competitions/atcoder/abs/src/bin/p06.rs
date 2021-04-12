use proconio::input;

fn main() {
    input! {
        n: u32,
        mut a: [i32; n],
    };

    // 昇順に並び替えた後で順番を反転させる
    a.sort();
    a.reverse();

    let mut sum = 0;
    for i in 0..a.len() {
        sum += if i % 2 == 0 { a[i] } else { -1 * a[i] };
    }
    println!("{}", sum);
}
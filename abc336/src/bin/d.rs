use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }

    // 横幅を累積高さと合わせる
    a.insert(0, 0);
    a.push(0);

    // 端は必ず１始まりなので、0で両端隣を初期化させておく
    let mut dl = vec![0; n + 2];
    let mut dr = vec![0; n + 2];

    for i in 1..n + 1 {
        dl[i] = a[i].min(dl[i - 1] + 1);
    }
    for i in (1..n + 1).rev() {
        dr[i] = a[i].min(dr[i + 1] + 1);
    }

    let mut answer = 1;
    for i in 1..n + 1 {
        let current_height = dl[i].min(dr[i]);
        answer = answer.max(current_height);
    }
    println!("{}", answer);
}

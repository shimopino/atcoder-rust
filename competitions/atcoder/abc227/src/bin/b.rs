use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i64; n],
    }

    let mut is_true = vec![false; n];
    for a in 1..1001 {
        for b in 1..1001 {
            let area = (4 * a * b) + (3 * a) + (3 * b);
            for i in 0..n {
                if s[i] == area {
                    is_true[i] = true;
                }
            }
        }
    }

    let answer = is_true.into_iter().filter(|x| *x == false).count();

    println!("{}", answer);
}

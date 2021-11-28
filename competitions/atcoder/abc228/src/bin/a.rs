use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize,
        x: usize,
    }

    let mut answer = {
        if s < t {
            // 同じ日付の場合
            if (s <= x) & (x < t) {
                true
            } else {
                false
            }
        } else {
            // 日付を跨ぐ場合
            if (x < t) || (s <= x) {
                true
            } else {
                false
            }
        }
    };

    println!("{}", if answer { "Yes" } else { "No" });
}

use proconio::input;
use proconio::marker::Chars;
use std::mem::swap;

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        tab: [(usize, usize, usize); q],
    }

    let (mut former, mut latter) = s.split_at_mut(n);

    for (t, mut a, mut b) in tab {
        if t == 1 {
            a -= 1;
            b -= 1;
            if b < n {
                former.swap(a, b);
            } else if a >= n {
                latter.swap(a - n, b - n);
            } else {
                let former_char = former[a];
                let latter_char = latter[b - n];
                former[a] = latter_char;
                latter[b - n] = former_char;
            }
        }

        if t == 2 {
            swap(&mut former, &mut latter);
        }
    }
    println!(
        "{}{}",
        former.iter().collect::<String>(),
        latter.iter().collect::<String>()
    );
}

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        tab: [[usize; 3]; q],
    }

    let mut rev_count = 0;
    for i in 0..q {
        let t: usize = tab[i][0];
        let a: usize = tab[i][1];
        let b: usize = tab[i][2];
        
        if t == 1 {
            s.swap(a - 1, b - 1);
        }

        if t == 2 {
            rev_count += 1;
        }
    }

    if rev_count % 2 == 0 {
        let (s1, s2) = s.split_at_mut(n);
        s1.swap_with_slice(s2);
    }

    let ans: String = s.into_iter().collect();
    println!("{}", ans);
}

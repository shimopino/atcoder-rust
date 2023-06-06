use proconio::{input, marker::Chars};

fn main() {
    input! {
      _x: usize,
      s: Chars,
      t: Chars
    }

    let is_similar = s.into_iter().zip(t).all(|(si, ti)| match (si, ti) {
        ('1', 'l') | ('l', '1') | ('o', '0') | ('0', 'o') => true,
        _ => si == ti,
    });

    println!("{}", if is_similar { "Yes" } else { "No" })
}

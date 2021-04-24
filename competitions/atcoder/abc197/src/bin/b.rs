use proconio::input;
use proconio::marker::{Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        x: usize,
        y: usize,
        s: [Chars; h],
    }

    let mut count = 1;

    // (x, y)のxの上側
    for i in (0..x-1).rev() {
        if s[i][y-1] == '#' {
            break;
        }
        count += 1;
    }

    // (x, y)のxの下側
    for i in (x..h) {
        if s[i][y-1] == '#' {
            break;
        }
        count += 1;
    }

    // (x, y)のyの左側
    for j in (0..y-1).rev() {
        if s[x-1][j] == '#' {
            break;
        }
        count += 1;
    }

    // (x, y)のxの下側
    for j in (y..w) {
        if s[x-1][j] == '#' {
            break;
        }
        count += 1;
    }

    println!("{}", count);
}
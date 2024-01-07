use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: i32,
        q: usize,
    }

    let mut points = VecDeque::new();
    for i in 1..=n {
        points.push_back((i, 0));
    }

    for _ in 0..q {
        input! { mode: char }

        if mode == '1' {
            input! { direction: char }
            let &(nx, ny) = points.front().unwrap();
            match direction {
                'R' => points.push_front((nx + 1, ny)),
                'L' => points.push_front((nx - 1, ny)),
                'U' => points.push_front((nx, ny + 1)),
                'D' => points.push_front((nx, ny - 1)),
                _ => unreachable!(),
            }
        } else {
            input! { p: usize }
            let &(nx, ny) = points.get(p - 1).unwrap();
            println!("{nx} {ny}");
        }
    }
}

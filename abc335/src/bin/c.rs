use proconio::input;

fn main() {
    input! {
        n: i32,
        q: usize,
    }

    let mut heads = (1..=n)
        .rev()
        .map(|current| (current, 0))
        .collect::<Vec<(i32, i32)>>();

    for _ in 0..q {
        input! { mode: usize, }

        if mode == 1 {
            input! { c: char }

            let &(nx, ny) = heads.last().unwrap();
            match c {
                'R' => heads.push((nx + 1, ny)),
                'L' => heads.push((nx - 1, ny)),
                'U' => heads.push((nx, ny + 1)),
                'D' => heads.push((nx, ny - 1)),
                _ => unreachable!(),
            }
        } else {
            input! { p: usize }

            let (x, y) = heads[heads.len() - p];
            println!("{x} {y}");
        }
    }
}

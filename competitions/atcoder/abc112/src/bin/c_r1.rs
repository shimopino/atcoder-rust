use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        xyh: [(i32, i32, i32); n],
    }

    for pos_y in 0..101 {
        for pos_x in 0..101 {
            let height = xyh
                .iter()
                .filter(|&(x, y, h)| h != &0)
                .map(|&(x, y, h)| h + (pos_x - x).abs() + (pos_y - y).abs())
                .next()
                .unwrap();
            println!("{}", height);
            if xyh
                .iter()
                .all(|&(x, y, h)| max(0, height - (pos_x - x).abs() - (pos_y - y).abs()) == h)
            {
                println!("{} {} {}", pos_x, pos_y, height);
            }
        }
    }
}

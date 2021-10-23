use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        mut s: Chars
    }

    let mut s_min: String = s.iter().collect();
    let mut s_max: String = s.iter().collect();
    let s_vec = VecDeque::from(s);

    let mut s_lshift = s_vec.clone();
    for _ in 0..s_lshift.len() {
        let pop_front = s_lshift.pop_front().unwrap();
        s_lshift.push_back(pop_front);

        let s_target: String = s_lshift.iter().collect();

        if s_target < s_min {
            s_min = s_target.clone();
        }

        if s_target > s_max {
            s_max = s_target.clone();
        }
    }


    let mut s_rshift = s_vec.clone();
    for _ in 0..s_rshift.len() {
        let pop_back = s_rshift.pop_back().unwrap();
        s_rshift.push_front(pop_back);

        let s_target: String = s_rshift.iter().collect();

        if s_target < s_min {
            s_min = s_target.clone();
        }

        if s_target > s_max {
            s_max = s_target.clone();
        }
    }

    println!("{}", s_min);
    println!("{}", s_max);
}

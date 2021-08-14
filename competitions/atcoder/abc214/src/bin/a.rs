use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let answer = {
        if n <= 125 {
            4
        } else if n <= 211 {
            6
        } else {
            8
        }
    };

    println!("{}", answer);
}
use proconio::input;

fn main() {
    input! {
        s: String
    }

    let s_vec: Vec<char> = s.chars().collect();
    let a = s_vec[0] == 'R';
    let b = s_vec[1] == 'R';
    let c = s_vec[2] == 'R';

    let mut count = 0;
    if (a && b && c) {
        println!("3");
    } else if ((a && b) || (b && c)) {
        println!("2");
    } else if (a || b || c) {
        println!("1");
    } else {
        println!("0");
    }
}

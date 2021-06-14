use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut ans = true;
    for (i, ia) in a.iter().enumerate() {
        if i+1 != *ia {
            ans = false;
        }
    }
    println!("{}", if ans {"Yes"} else {"No"});
}
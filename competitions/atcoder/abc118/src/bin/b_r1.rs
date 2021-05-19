use proconio::input;

fn main() {
    input! {
        n: i32,
        m: usize,
        a[[usize]; n],
    }

    let ans = (1..m + 1).filter(|x| a.iter().all(|ai| ai.contains(x)).count());
    println!("{}", ans);
}

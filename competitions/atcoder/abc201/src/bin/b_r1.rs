use proconio::input;

fn main() {
    input! {
        n: usize,
        mut st: [(String, i32); n],
    }

    st.sort_by_key(|st| st.1);
    println!("{}", st[n - 2].0);
}
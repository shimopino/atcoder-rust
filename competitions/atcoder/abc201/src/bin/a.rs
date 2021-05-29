use proconio::input;

fn main() {
    input! {
        mut a: [i32; 3],
    }

    a.sort();
    let ans = if a[2] - a[1] == a[1] - a[0] {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}

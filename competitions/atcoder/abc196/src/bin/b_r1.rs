use proconio::input;

fn main() {
    input! {
        mut x: String,
    }

    // findは指定したcharが最初に出現するインデックスを返す
    match x.find(".") {
        // 指定した文字数までTrimmingする
        Some(n) => x.truncate(n),
        None => (),
    }
    println!("{}", x);
}
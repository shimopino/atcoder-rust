use proconio::input;

fn main() {
    input! {
        x: String,
    }

    let v: Vec<&str> = x.split(".").collect();
    for s in v[0].chars() {
        print!("{}", s);
    }
    println!("");
}
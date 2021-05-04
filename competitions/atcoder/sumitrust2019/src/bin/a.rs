use proconio::input;

fn main() {
    input! {
        md: [(u32, u32); 2],
    }

    if md[0].1 > md[1].1 {
        println!("1");
    } else {
        println!("0");
    }
}
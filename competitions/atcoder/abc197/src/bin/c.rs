use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let a_bin: Vec<String> = a.iter()
                              .map(|i| format!("{:0>30b}", i))
                              .collect();

    for i in a_bin.iter() {
        println!("{}", i);
    }

    println!("{}", 3 & 5);
}
fn main() {
    let mut s = String::from("hello");

    s.push_str(" world");

    println!("{}", s);

    let s2 = s;
    println!("{}", s2);

    let s3 = s2.clone();
    println!("{}", s3);
}

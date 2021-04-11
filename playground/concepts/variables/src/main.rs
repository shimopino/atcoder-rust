fn main() {
    // let x = 5
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let tup: (i32, f64, u8) = (500, 0.1, 1);

    let (a, b, c) = tup;

    println!("The value of a, b, c is: {}, {}, {}", a, b, c);

    println!("tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);
}

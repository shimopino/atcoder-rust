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

    let array = [1, 2, 3, 4, 5];
    let first = array[0];
    let last = array[array.len() - 1];

    println!("first: {}, last: {}", first, last);

    // 問題9
    another_function();

    // 問題10
    print_age(100);

    // 問題11
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    // 問題12
    let x = plus_one(5);
    println!("The value of x is: {}", x);

    println!("{} - {}", 'A', b'A');
    println!("{} - {}", 'Z', b'Z');
    println!("{} - {}", 'a', b'a');
    println!("{} - {}", 'z', b'z');
    println!("{} - {}", '0', b'0');
    println!("{} - {}", '9', b'9');
}

fn another_function() {
    println!("Another Function");
}

fn print_age(age: u8) {
    println!("Your age is {}", age);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

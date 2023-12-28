use proconio::input;

fn main() {
    input! {
      mut n: i32,
    }

    let mut answer = String::new();
    for index in (0..10).rev() {
        let target = 2_i32.pow(index);
        let sho = n / target;
        answer = format!("{}{}", answer, sho);
        n = n % target;
    }

    println!("{}", answer);
}

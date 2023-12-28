use proconio::input;

fn main() {
    input! {
      n: usize,
      x: usize,
      a: [usize; n],
    }

    let mut found = false;
    for &ai in a.iter() {
        if x == ai {
            found = true;
        }
    }

    println!("{}", if found { "Yes" } else { "No" });
}

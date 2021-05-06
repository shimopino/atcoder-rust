use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = s.chars()
               .fold(700, |acc, c| if c == 'o' {
                   acc + 100
               } else {
                   acc
               });
    println!("{}", ans);
}
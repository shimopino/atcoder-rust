use proconio::input;

fn main() {
    input! {
        n: u32,
        m: usize,
        pair: [(usize, char); m],
    }

    let ans = (0..10i32.pow(n))
            .filter(|x| {
                let x = x.to_string().as_bytes().to_vec();
                x.len() == n as usize
                && pair
                  .iter()
                  .all(|(s, c)| matches!(x.get(*s-1), Some(x) if *x as char == *c))
            })
            .next()
            .unwrap_or(-1);
    println!("{}", ans);
}
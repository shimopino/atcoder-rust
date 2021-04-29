use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; 2_u32.pow(n)],
    }

    let half = 2_u32.pow(n - 1) as usize;
    let block1 = a[..half].iter().max().unwrap();
    let block2 = a[half..].iter().max().unwrap();

    let final_loser = block1.min(block2);
    let loser_pos = a.iter().position(|x| x == final_loser).unwrap();

    println!("{}", loser_pos + 1);
}
use proconio::input;

fn main() {
    input! {
        n: u32,
        mut a: [u32; n]
    };
    let mut result: u32 = 0;
    loop {
        a = a.iter()
            .filter_map(|x| if x % 2 == 0 { Some(x / 2) } else { None })
            .collect();

        if a.len() < n as usize {
            break;
        }

        result += 1;
    }
    println!("{}", result);
}
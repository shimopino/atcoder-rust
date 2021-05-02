use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        abcd: Chars,
    }

    let len = 3;

    let v: Vec<i32> = abcd.iter()
                          .map(|c| c.to_digit(10).unwrap() as i32)
                          .collect();
    
    // bit全探索
    for bit in 0..(1 << len) {
        let mut res: i32 = v[0];
        let mut ops = vec!["+", "+", "+"];
        // 1桁ずつ検証していく
        for n in 0..len {
            if bit & (1 << n) == 0b0 {
                res += v[n + 1];
            } else {
                ops[n] = "-";
                res -= v[n + 1];
            }
        }
        if res == 7 {
            println!("{}{}{}{}{}{}{}=7", v[0], ops[0], v[1], ops[1], v[2], ops[2], v[3]);
            break;
        }
    }
}

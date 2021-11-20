use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i64; n],
    }

    let mut answer = 0;
    for i in 0..n {
        let mut flag = 0;
        for a in 1..1001 {
            for b in 1..1001 {
                if s[i] == 4*a*b + 3*a + 3*b {
                    flag = 1;
                }
            }
        }
        if flag == 0 { answer += 1; }
    }

    println!("{}", answer);
}

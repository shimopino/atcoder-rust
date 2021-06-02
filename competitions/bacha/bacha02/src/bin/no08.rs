use proconio::input;

fn main() {
    input! {
        c: [[i32; 3]; 3],
    }

    for i in 0..101 {
        for j in 0..101 {
            for k in 0..101 {
                let a1 = i;
                let a2 = j;
                let a3 = k;

                let b1 = c[0][0] - a1;
                let b2 = c[1][1] - a2;
                let b3 = c[2][2] - a3;

                if a1 + b1 == c[0][0]
                    && a1 + b2 == c[0][1]
                    && a1 + b3 == c[0][2]
                    && a2 + b1 == c[1][0]
                    && a2 + b2 == c[1][1]
                    && a2 + b3 == c[1][2]
                    && a3 + b1 == c[2][0]
                    && a3 + b2 == c[2][1]
                    && a3 + b3 == c[2][2]
                {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}

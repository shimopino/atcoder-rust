use proconio::input;

fn main() {
    input! {
        a: [[u32; 3]; 3],
        n: usize,
        b: [u32; n],
    }

    let mut bingo_book = vec![vec![false, false, false]; 3];
    for call in 0..n {
        for i in 0..3 {
            for j in 0..3 {
                if a[i][j] == b[call] {
                    bingo_book[i][j] = true;
                }
            }
        }
    }

    let mut ans = "No";
    for i in 0..3 {
        if bingo_book[i][0] && bingo_book[i][1] && bingo_book[i][2] {
            ans = "Yes"
        }
    }
    for j in 0..3 {
        if bingo_book[0][j] && bingo_book[1][j] && bingo_book[2][j] {
            ans = "Yes"
        }
    }
    if bingo_book[0][0] && bingo_book[1][1] && bingo_book[2][2] {
        ans = "Yes"
    }
    if bingo_book[0][2] && bingo_book[1][1] && bingo_book[2][0] {
        ans = "Yes"
    }

    println!("{}", ans);
}
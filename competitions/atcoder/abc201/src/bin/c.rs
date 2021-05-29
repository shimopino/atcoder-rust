use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    // 例えば "1234" の場合、次のように出現回数を数字毎に管理し、フラグと対応付けする
    // index:  ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
    // number: [ 0 ,  1,   1,   1,   1,   0,   0,   0,   0,   0 ]
    // s     : ['x', 'o', 'o', 'o', 'o', 'x', '?', '?', 'x', '?']

    let mut count = 0;
    for i in 0..10000 {
        // 入力されるフラグ数と数値を合わせる
        let number_string = format!("{:04}", i);
        let mut number_count = vec![0; 10];
        for inumber in number_string.chars() {
            let index= inumber.to_digit(10).unwrap();
            number_count[index as usize] += 1;
        }

        // 0～9の条件を全て通る場合にカウントする
        let mut flag = true;
        for j in 0..10 {
            if s[j] == 'o' && number_count[j] == 0 {
                flag = false;
                break;
            } 
            
            if s[j] == 'x' && number_count[j] > 0 {
                flag = false;
                break;
            }
        }

        if flag {
            count += 1;
        }
    }
    println!("{}", count);
}
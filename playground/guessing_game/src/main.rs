// 標準ライブラリに存在する入出力ライブラリ（io）をスコープに導入
// 標準ライブラリにはデフォルトで使用できるものもある
// https://doc.rust-lang.org/std/prelude/index.html
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is {}", secret_number);

    loop {

        println!("Please input your guess.");
    
        // let を使用して変数を生成する
        // Rustは変数は標準で不変（immutable）である
        // mut をつけて変数を可変にする
        // 変数 guess をString型のオブジェクトに束縛する
        // String型は、サイズ可変、UTF-8エンコードされている
        // :: を使用して型に定義されている static なメソッドにアクセスする
        // new() は空の文字列を新たに生成する
        let mut guess = String::new();
    
        // stdin は std::io::Stdin オブジェクトを返す
        // このオブジェクトはターミナルへの標準入力へのハンドルをあらわす
        // real_line はユーザの標準入力への値を文字列に格納する
        // 格納先の変数は可変である必要がある
        // & をつけて引数が参照であることを示し、宣言済みの変数にアクセスする
        // その際に参照も標準で可変であるため、mut をつける
        io::stdin().read_line(&mut guess)
            // real_line は文字列の格納だけではなく io::Result 型の値を返す
            // この型は enum であり、列挙子は Ok か Err
            // Ok -> expectメソッドは Ok 列挙子が保持する値を取り出して返す
            // Err -> expectメソッドはプログラムをクラッシュさせる
            .expect("Failed to read line");
    
        // Rustでは宣言済みの変数に対して、新しい値で guess の値を覆い隠す
        // シャーディングが許容されている
        // 文字列型の trim() で両端の空白を全て除去する
        // ユーザーが標準入力を指定して Enter を実行すると文字列には
        // 改行文字 \n が含まれるが、trim() で削除できる
        // parse() で文字列を数値に変換できるが、必ず型を明示する必要あり
        // 
        // let guess: u32 = guess.trim().parse()
        //     // 数値以外を入力すると、返り値で Err が返り、expect内の引数
        //     // がメッセージとして表示される
        //     .expect("Please Type a number");

        // expect メソッド呼び出しから match 式に変更することで
        // エラー処理を行うことが可能になる
        // parse() は列挙子として Ok か Err を返す
        let guess: u32 = guess.trim().parse();
    
        // プレースホルダー {} を使用して変数を文字列に代入できる
        println!("You guessed: {}", guess);
    
        // 文字列型の cmp メソッドは2価を比較する
        // 引数には、比較したいものへの参照をとる
        // match 式は、guess.cmp の値がマッチする式を実行する
        // $secret_number は数値型なので、文字列の引数と型の不一致になる
        // Rustの数値がたは、i32:32ビット数字、u32:32ビット非負数字、i64:64ビット数字
        // 何も設定しない場合は i32 になる
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}


// use std::io;

// let mut input = String::new();
// match io::stdin().read_line(&mut input) {
//     Ok(n) => {
//         println!("{} bytes read", n);
//         println!("{}", input);
//     }
//     Err(error) => println!("error: {}", error),
// }
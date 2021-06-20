// 参考資料
// https://stackoverflow.com/questions/45724517/how-to-iterate-through-a-hashmap-print-the-key-value-and-remove-the-value-in-ru
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
        m: usize,
        t: [usize; m],
    }

    let mut set_a = HashMap::new();
    let mut set_b = HashMap::new();

    for id in d {
        // HashMapに対応する値がない場合はデフォルト値 (0) を挿入
        // 存在する場合は mutable な参照を返す
        *set_a.entry(id).or_insert(0) += 1;
    }

    for it in t {
        // HashMapに対応する値がない場合はデフォルト値 (0) を挿入
        // 存在する場合は mutable な参照を返す
        *set_b.entry(it).or_insert(0) += 1;
    }

    let mut ans = true;
    for (key, value) in set_b {
        // 所定の難易度の問題が存在しない場合
        if !set_a.contains_key(&key) {
            ans = false;
            break;
        }

        // 所定の難易度の問題集の数が満たない場合
        if set_a[&key] < value {
            ans = false;
            break;
        }
    }
    
    println!("{}", if ans {"YES"} else {"NO"});
}
fn main() {
    let list = vec!["A", "B", "C", "D"];
    let n = list.len() as u32;

    println!("{0:02}, {0:05b}", 1 << 0); // 01: 00001 
    println!("{0:02}, {0:05b}", 1 << 1); // 02: 00010
    println!("{0:02}, {0:05b}", 1 << 2); // 04: 00100
    println!("{0:02}, {0:05b}", 1 << 3); // 08: 01000
    println!("{0:02}, {0:05b}", 1 << 4); // 16: 10000

    for bit in 0..(1 << n) {
        println!("{:02}: {:04b}: {:#04b}", bit, bit, bit);
    }

    for i in 0..n {
        println!("{:04b}", (1 << i));
    }

    for bit in 0..(1 << n) {
        for i in 0..n {
            println!("{:04b} & {:04b} = {:04b}, {}", bit, (1 << i), bit & (1 << i), bit & (1 << i) != 0b0);
        }
    }

    // 以下では [0000, 0001, 0010, 0011, 0100, ..., 1111] と探索する
    for bit in 0..(1 << 4) {
        let mut s = Vec::new();
        // 以下では bit値の i桁目が1である配列要素を抽出する
        for i in 0..4 {
            // AND演算を実施する
            if bit & (1 << i) != 0b0 {
                s.push(list[i])
            }
        }

        print!("{:04b} -> ", bit);
        for i in s.iter() {
            print!("{} ", i);
        }
        println!("");
    }

    for bit in 0..(1 << n) {
        let sub_list = (0..n).filter(|x| (bit & (1 << x)) != 0)
                             .map(|x| list[x as usize]);

        sub_list.for_each(|x| {
            print!("{} ", x);
        });
        println!("");
    }
}
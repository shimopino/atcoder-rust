fn main() {
    let list = vec!["A", "B", "C", "D"];
    let n = list.len();

    for bit in 0..(1 << n) {
        println!("bit: {}", bit);
        let subset = (0..n)
            .filter(|x| bit & (1 << x) != 0)
            .map(|x| list[x])
            .collect::<Vec<_>>();

        println!("{:?}", subset);
    }
}

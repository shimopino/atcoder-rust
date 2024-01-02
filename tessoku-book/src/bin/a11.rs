use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n],
    }

    println!("{}", binary_search(&a, x) + 1);
}

fn binary_search(arr: &[i64], x: i64) -> usize {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut answer = 0;
    while left <= right {
        let mid = (left + right) / 2;
        match x.cmp(&arr[mid]) {
            std::cmp::Ordering::Less => right = mid - 1,
            std::cmp::Ordering::Equal => {
                answer = mid;
                break;
            }
            std::cmp::Ordering::Greater => left = mid + 1,
        }
    }
    answer
}

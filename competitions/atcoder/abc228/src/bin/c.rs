use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [[usize; 3]; n],
    }

    let mut total_3 = vec![0; n];
    for (index, points) in p.into_iter().enumerate() {
        total_3[index] = points.into_iter().sum();
    };

    let mut total_3_sort = total_3.clone();
    total_3_sort.sort();
    total_3_sort.reverse();

    for point in total_3.into_iter() {
        if point + 300 >= total_3_sort[k - 1] {
            println!("Yes");
        } else {
            println!("No");
        }
    }

    return;
}

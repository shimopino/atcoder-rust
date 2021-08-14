use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    let mut min_pos = n;
    let mut min_value = std::usize::MAX;
    for (i, it) in t.iter().enumerate() {
        if *it <= min_value {
            min_value = *it;
            min_pos = i;
        }
    }

    let mut current = min_pos;
    let mut answer = vec![0; n];
    // 例1の場合には、3が格納される
    answer[current] = s[current].min(t[current]);
    for _ in 1..n {

        // 1回目: current: 0, next: 1
        // 2回目: current: 1, next: 2
        let next = {
            if current + 1 == n {
                0
            } else {
                current + 1
            }
        };

        // 1回目: 3 + 4 >= 10 なので answer[next] = 7
        // 2回目: 7 + 1 >= 100 なので answer[next] = 8
        if answer[current] + s[current] >= t[next] {
            answer[next] = t[next];
        } else {
            answer[next] = answer[current] + s[current];
        }

        current = next
    }

    for i in answer {
        println!("{}", i);
    }
}
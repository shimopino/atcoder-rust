use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    dfs(n, 0, "".to_string());
}

fn dfs(
    n: usize,
    depth: usize,
    target: String
) {
    if depth == n {
        if correct(&target) {
            println!("{}", target);
        }
        return;
    }

    for next in vec!["(", ")"] {
        dfs(n, depth + 1, format!("{}{}", target, next));
    }
}

fn correct(target: &String) -> bool {

    let mut count = 0;

    for ic in target.chars() {
        if ic == '(' {
            count += 1;
        } else {
            count -= 1;
        }

        if count < 0 {
            return false
        }
    }

    if count == 0 {
        return true;
    } else {
        return false;
    }
}
use std::string::String;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    println!(
        "{}",
        if s.chars().take(4).collect::<String>() == "YAKI" {
            "Yes"
        } else {
            "No"
        }
    );
}
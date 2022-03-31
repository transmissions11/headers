use std::env;

fn main() {
    let input = env::args().collect::<Vec<String>>()[1..].join(" ");

    let mut space = String::from("");

    while (64 - (space.len() + input.len())) > space.len() {
        space.push_str(" ")
    }

    println!(
        "{}\n{}{}{}\n{}",
        "    /*//////////////////////////////////////////////////////////////",
        "    ",
        space,
        input.to_uppercase(),
        "    //////////////////////////////////////////////////////////////*/"
    );
}

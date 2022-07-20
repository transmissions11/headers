use std::env;
use std::{thread, time};
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let input = env::args().collect::<Vec<String>>()[1..].join(" ");

    let mut space = String::from("");

    while (64 - (space.len() + input.len())) > space.len() {
        space.push_str(" ")
    }

    let output = format!(
        "{}\n{}{}{}\n{}",
        "    /*//////////////////////////////////////////////////////////////",
        "    ",
        space,
        input.to_uppercase(),
        "    //////////////////////////////////////////////////////////////*/"
    );

    println!("{}", output); // Print the header to console.

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
    thread::sleep(time::Duration::from_secs(1)); // This is for Linux users.
}

use std::env;

use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let input = env::args().collect::<Vec<String>>()[1..].join(" ");

    let output = format!(
        "{}\n{}{}{}\n{}",
        "    /*//////////////////////////////////////////////////////////////",
        "    ",
        (0..(64 - input.len()) / 2).map(|_| " ").collect::<String>(),
        input.to_uppercase(),
        "    //////////////////////////////////////////////////////////////*/"
    );

    println!("{}", output); // Print the header to console.

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
}

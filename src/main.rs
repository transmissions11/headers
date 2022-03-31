use std::env;

struct Header {
    top_block: String,
    bottom_block: String,
    padding: String,
    input: String,
}

impl Header {
    fn new(input: String, character: char) -> Header {
        let mut top_block = String::from("/*");
        let mut bottom_block = String::new();

        let mut i = 0;
        while i < 62 {
            top_block.push(character);
            bottom_block.push(character);
            i += 1;
        }

        bottom_block.push_str("*/");

        let mut space = String::from("");

        for _i in 1..(&top_block.len() - &input.len()) / 2 {
            space.push_str(" ")
        }

        Header {
            top_block: top_block,
            bottom_block: bottom_block,
            padding: space,
            input: input.to_uppercase()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1];
    let character = &args[2];

    let header = Header::new(input.to_string(), character.chars().nth(0).unwrap());

    println!("{}\n{}{}\n{}", header.top_block, header.padding, header.input, header.bottom_block);
}

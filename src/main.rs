#![feature(ascii_char)]

//mod commands;
mod gmail;
mod terminal;
mod util;

fn main() {
    // let text = "Hello, world!";
    let text = std::fs::read_to_string("./src/util/sha256.rs").expect("file nçot found");
    // let txt = std::env::args().nth(1).unwrap();
    // println!("{}", txt);
    let buffer = text.as_bytes();
    // println!("{}", util::base64::url_encode(&buffer))
    let hash = util::sha256::sha256(buffer);
    // println!("{:?}", filled);
    // println!("{} bits", filled.len() * 8);
    println!(
        "{}",
        hash.into_iter()
            .map(|v| format!("{:x}", v))
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v);
                acc
            })
    );
}

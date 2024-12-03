#![feature(ascii_char)]

//mod commands;
mod gmail;
mod terminal;
mod util;

fn main() {
    let text = "1234567890123456789012345678901234567890123456789012345KJDFHLSHDJMFHMLDSKFJMLKJDGFLMKJFDSMLKJFMLDKJFDJLJDFSJLJDSKLFJ";
    // let txt = std::env::args().nth(1).unwrap();
    // println!("{}", txt);
    let buffer = text.as_bytes();
    // println!("{}", util::base64::url_encode(&buffer))
    let filled = util::sha256::fill_bits::<512>(buffer);
    println!("{:?}", filled);
    println!("{} bits", filled.len() * 8);
}

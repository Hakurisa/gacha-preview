use std::io;
use text_io::read;
mod gamba_brain;
fn main() {
    println!("Enter your desired character!\nYae Miko, Keqing, Tighnari, Mona, Jean, Diluc, Qiqi, Thoma, Layla, Shikanoin Heizou");
    let mut character = String::new();
    let roll_amount: u32;
    let pity_4star: u8;
    let pity_5star: u32;
    io::stdin().read_line(&mut character).expect("Failed to read line");
    println!("How many rolls?");
    roll_amount = read!();
    println!("What's your 4 star pity?");
    pity_4star = read!();
    println!("What's your 5 star pity?");
    pity_5star = read!();

    gamba_brain::gamba_brain(&character, roll_amount, pity_4star, pity_5star);

}

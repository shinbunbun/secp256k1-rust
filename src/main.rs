use std::{io, process::exit};

use rug::{integer::Order, Integer};

use crate::{hash::create_sha256_from_string, private_key::PrivateKey, secp256k1::Secp256k1};

mod field_element;
mod hash;
mod point;
mod pow;
mod private_key;
mod secp256k1;
mod signature;

fn main() {
    println!("Hello, Secp256k1!\n");

    println!("1. Input evaluation command [V]erify or [S]ign: ");
    let mut eval_command = String::new();
    io::stdin().read_line(&mut eval_command).unwrap();

    match eval_command.trim() {
        "V" | "v" => {
            println!("v");
        }
        "S" | "s" => sign(),
        _ => {
            println!("\nInvalid command");
            exit(1);
        }
    }
}

fn sign() {
    println!("1. Please input secret key: ");
    let mut secret = String::new();
    io::stdin().read_line(&mut secret).unwrap();
    secret = secret.trim().to_string();

    let secret = Integer::from_digits(create_sha256_from_string(&secret).as_slice(), Order::LsfBe);
    let private_key = PrivateKey::new(secret, Secp256k1::get_g());
    let sec256 = Secp256k1::new(Some(private_key), None);

    println!("2. Please input message: ");
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();
    message = message.trim().to_string();
    let message =
        Integer::from_digits(create_sha256_from_string(&message).as_slice(), Order::LsfBe);

    let k = sec256.deterministic_k(message.clone());
    let signature = sec256.sign(message, k);

    println!("{:?}", signature);
}

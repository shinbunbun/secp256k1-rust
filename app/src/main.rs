use std::{io, process::exit, str::FromStr};

use elliptic_curve::{Ecdsa, Signature};
use rug::{integer::Order, Integer};

use crate::{hash::create_sha256_from_string, secp256k1::Secp256k1};

mod hash;
mod secp256k1;

fn main() {
    println!("Hello, Secp256k1!\n");

    println!("1. Input evaluation command [V]erify or [S]ign: ");
    let mut eval_command = String::new();
    io::stdin().read_line(&mut eval_command).unwrap();

    match eval_command.trim() {
        "V" | "v" => {
            verify();
        }
        "S" | "s" => sign(),
        _ => {
            println!("\nInvalid command");
            exit(1);
        }
    }
}

fn read_line(input: &mut String) -> &str {
    io::stdin().read_line(input).unwrap();
    input.trim()
}

fn verify() {
    println!("\n2. Please input the public key x: ");
    let mut x = String::new();
    let x = read_line(&mut x);

    println!("\n3. Please input the public key y: ");
    let mut y = String::new();
    let y = read_line(&mut y);

    let public_key = Secp256k1::generate_public_key_from_coord(
        Integer::from_str(x).unwrap(),
        Integer::from_str(y).unwrap(),
    );
    let sec256 = Secp256k1::new(None, public_key);

    println!("\n4. Please input the message: ");
    let mut message = String::new();
    let message = Integer::from_digits(
        create_sha256_from_string(read_line(&mut message)).as_slice(),
        Order::MsfBe,
    );

    println!("\n5. Please input the signature r: ");
    let mut r = String::new();
    let r = Integer::from_str(read_line(&mut r)).unwrap();

    println!("\n6. Please input the signature s: ");
    let mut s = String::new();
    let s = Integer::from_str(read_line(&mut s)).unwrap();

    let signature = Signature::new(r, s);
    let result = sec256.verify(message, signature);

    if result {
        println!("\nVerification succeeded!");
    } else {
        println!("\nVerification failed!");
    }
}

fn sign() {
    println!("\n2. Please input secret: ");
    let mut secret = String::new();

    let sec256 = Secp256k1::generate_key_pair_from_secret(read_line(&mut secret));

    println!(
        "\nPublic Key: (x, y) = ({}, {})",
        sec256.public_key.x.clone().unwrap().num,
        sec256.public_key.y.clone().unwrap().num
    );

    println!("\n3. Please input message: ");
    let mut message = String::new();
    let message = Integer::from_digits(
        create_sha256_from_string(read_line(&mut message)).as_slice(),
        Order::MsfBe,
    );

    let k = sec256.deterministic_k(message.clone());
    let signature = sec256.sign(message, k);

    println!("\n{:?}", signature);
}

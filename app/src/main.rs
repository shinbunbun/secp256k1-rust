use std::{io, process::exit, str::FromStr};

use elliptic_curve::Signature;
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

fn verify() {
    println!("\n2. Please input the public key x: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x = Some(Secp256k1::create_field_element(
        Integer::from_str(x.trim()).unwrap(),
    ));

    println!("\n3. Please input the public key y: ");
    let mut y = String::new();
    io::stdin().read_line(&mut y).unwrap();
    let y = Some(Secp256k1::create_field_element(
        Integer::from_str(y.trim()).unwrap(),
    ));

    let public_key = Secp256k1::create_point(x, y);
    let sec256 = Secp256k1::new_with_public_key(public_key);

    println!("\n4. Please input the message: ");
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();
    let message = message.trim();
    let message = Integer::from_digits(create_sha256_from_string(message).as_slice(), Order::MsfBe);

    println!("\n5. Please input the signature r: ");
    let mut r = String::new();
    io::stdin().read_line(&mut r).unwrap();
    let r = Integer::from_str(r.trim()).unwrap();

    println!("\n6. Please input the signature s: ");
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = Integer::from_str(s.trim()).unwrap();

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
    io::stdin().read_line(&mut secret).unwrap();

    let sec256 = Secp256k1::new_with_secret(secret.trim());

    println!(
        "\nPublic Key: (x, y) = ({}, {})",
        sec256.public_key.x.clone().unwrap().num,
        sec256.public_key.y.clone().unwrap().num
    );

    println!("\n3. Please input message: ");
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();
    let message = Integer::from_digits(
        create_sha256_from_string(message.trim()).as_slice(),
        Order::MsfBe,
    );

    let k = sec256.deterministic_k(message.clone());
    let signature = sec256.sign(message, k);

    println!("\n{:?}", signature);
}

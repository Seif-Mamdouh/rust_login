use std::io::{self, Write};

fn sign_up() -> (String, String) {
    println!("Enter your username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim().to_string();

    println!("Enter your password: ");
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim().to_string();

    (username, password)
}

fn main() {
    println!("Welcome to Knowledge");
    println!("---------------------");

    println!("Do you have a login? (y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    match input {
        "y" => {
            println!("Please enter your login details");
            // create_login();
        },
        "n" => {
            println!("Sign up");
            let (username, password) = sign_up();
            println!("Welcome {}!", username, );
        },
        _ => {
            println!("Invalid input. Please enter 'y' or 'n'.");
        }
    }
}


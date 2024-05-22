use ssh2::{Session, Channel};
use std::net::TcpStream;
use std::io::Read;



fn login() -> (String, String) {
    println!("Enter your username: ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    println!("Enter your password: ");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    (username.to_string(), password.to_string())
}



fn sign_up() -> (String, String){
    println!("Enter your username: ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    println!("Enter your password: ");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    (username.to_string(), password.to_string())

}


fn main() {

    println!("Welcome to Knowledge");
    println!("---------------------");


    println!("Do you have a login? (y/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    match input {
        "y" => {
            println!("Please create a login");
            create_login();
        },
        "n" => {
            println!("Sign up");
            sign_up();
        },
    }

}

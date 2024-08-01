// Mod
mod parser;
mod utils;
mod splinter;

// Libs
use regex::Regex;

#[tokio::main]
async fn main() {
    utils::banner();
    utils::copyright();

    let req = parser::initialize();

    match req.mode.as_ref() {
        "mail" => {
            let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
            if email_regex.is_match(&req.value) {
                println!("Email is valid");
            } else {
                println!("Email is not valid");
                std::process::exit(1);
            }
            proton::mail::req(&req.value).await.expect("Error while checking ProtonMail");
        },
        "ip" => {
            let ip_regex = Regex::new(r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$").unwrap();
            if ip_regex.is_match(&req.value) {
                println!("IP address is valid");
            } else {
                println!("IP address is not valid");
                std::process::exit(1);
            }
            proton::vpn::req(&req.value).await.expect("Error while checking ProtonVPN");
        },
        _ => {
            println!("You need to choose at least 1 mode : Mail or IP");
            std::process::exit(1)
        }
    }
}
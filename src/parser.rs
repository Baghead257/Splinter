use clap::{Arg, Command, value_parser};

pub struct Request {
    pub mode: String,
    pub value: String
}

pub fn initialize() -> Request {
    let app = Command::new("Proton")
        .version("1.0")
        .author("Unknown")
        .about("Investigate Protonmail accounts and ProtonVPN IP addresses")
        .arg(
            Arg::new("mail")
                .conflicts_with("ip")
                .short('m')
                .long("mail")
                .value_parser(value_parser!(String))
                .help_heading("GENERAL options")
                .help("Gather information about ProtonMail"),
        )
        .arg(
            Arg::new("ip")
                .conflicts_with("mail")
                .short('i')
                .long("ip")
                .value_parser(value_parser!(String))
                .help_heading("GENERAL options")
                .help("Check if IP is from ProtonVPN"),
        )
        .get_matches();

    let mut mode = String::new();
    let mut value = String::new();

    if let Some(cli_value) = app.get_one::<String>("mail") {
        mode = "mail".to_string();
        value = cli_value.to_string();
    }

    if let Some(cli_value) = app.get_one::<String>("ip") {
        mode = "ip".to_string();
        value = cli_value.to_string();
    }

    let req = Request {
        mode,
        value,
    };

    req

}
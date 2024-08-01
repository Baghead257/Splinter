use reqwest::Client;

pub async fn req(ip: &str) -> Result<(), reqwest::Error> {

    let client = Client::new();
    let url = "https://api.protonmail.ch/vpn/logicals";

    let response = client.get(url)
        .header("User-Agent", "")
        .send()
        .await?;

    let body = response.text().await?;

    let ip = format!("{ip}\"");

    if body.contains(&ip) {
        println!("The ip is from ProtonVPN")
    }
    else {
        println!("This is not a ProtonVPN IP")
    }

    Ok(())
}
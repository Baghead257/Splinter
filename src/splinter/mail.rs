use reqwest::Client;
use regex::Regex;
use chrono::{TimeZone, Utc};

pub async fn req(mail: &str) -> Result<(), reqwest::Error> {

    let client = Client::new();
    let url = format!("https://api.protonmail.ch/pks/lookup?op=index&search={}", mail);
    let url_key = format!("https://api.protonmail.ch/pks/lookup?op=get&search={}", mail);

    let response = client.get(url)
        .header("User-Agent", "")
        .send()
        .await?;

    let response_key = client.get(url_key)
        .header("User-Agent", "")
        .send()
        .await?;

    let body = response.text().await?;
    let body_key = response_key.text().await?;

    if body.contains("info:1:0") {
        println!("Bad mail")
    }
    else if body.contains("info:1:1") {

        let mut timestampd: i64 = 0;

        let regex_ptrn = vec![
            Regex::new(r"2048:(.*)::").unwrap(),
            Regex::new(r"4096:(.*)::").unwrap(),
            Regex::new(r"22::(.*)::").unwrap(),
        ];

        for regexp in regex_ptrn {
            if let Some(caps) = regexp.captures(&body) {
                if let Some(timestamp) = caps.get(1) {
                    let timestamp = timestamp.as_str();
                    timestampd= timestamp.parse().unwrap();
                }
            }
        }

        let dt = Utc.timestamp_opt(timestampd, 0).unwrap();

        println!("UID: {}", mail);
        println!("Creation Date: {}", dt);

    }
    else {
        println!("Can't query the API");
    }
    println!("{}", body_key);

    Ok(())
}
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().cookie_store(true).build()?;
    let base_url = "http://192.168.1.79/DVWA/";
    match login(&client, base_url) {
        Ok(Some(success)) => {
            println!("Login {}", success);
            set_security_level(&client, base_url)?;
            let xss_reflected = xss_reflected_scanner(&client, base_url)?;
            match xss_reflected {
                Some(xss) => println!("XSS Reflected {}", xss),
                None => println!("No XSS Reflected"),
            }
        }
        Ok(None) => println!("Login failed"),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

fn login(client: &Client, base_url: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let login_url = format!("{}{}", base_url, "login.php");
    let password = "password";

    let login_page_response = client.get(&login_url).send()?;
    let login_page_html = login_page_response.text()?;
    let user_token = get_usertoken(&login_page_html)?;
    //println!("User Token: {}", user_token);
    let response = client
        .post(&login_url)
        .form(&[
            ("username", "admin"),
            ("password", &password),
            ("Login", "Login"),
            ("user_token", &user_token),
        ])
        .send()?;
    let response_html = response.text()?;
    //println!("{}", response_html);
    if response_html.contains("Login failed") {
        Ok(None)
    } else {
        Ok(Some("successful ...".to_string()))
    }
}

fn set_security_level(client: &Client, base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let sec_level_url = format!("{}{}", base_url, "security.php");
    let sec_level_response = client.get(&sec_level_url).send()?;
    let sec_level_html_response = sec_level_response.text().unwrap();
    //println!("{}", sec_level_html_response);
    let user_token = get_usertoken(sec_level_html_response.as_str())?;

    let mut sec_level_params = HashMap::new();
    sec_level_params.insert("security", "low");
    sec_level_params.insert("seclev_submit", "Submit");
    sec_level_params.insert("user_token", &user_token);

    let _sec_level_response = client.post(&sec_level_url).form(&sec_level_params).send()?;
    Ok(())
}

fn xss_reflected_scanner(
    client: &Client,
    base_url: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let xss_reflected_url = format!("{}{}", base_url, "vulnerabilities/xss_r/");

    let mut xss_params: HashMap<&str, &str> = HashMap::new();
    xss_params.insert("name", "<script>alert('XSS');</script>");
    xss_params.insert("submit", "Submit");

    let request = client.get(&xss_reflected_url).query(&xss_params).build()?;

    //let xss_reflected_response = client.get(&xss_reflected_url).query(&xss_params).send()?;
    let full_url = request.url().to_string();
    let xss_reflected_response = client.execute(request)?;
    let xss_reflected_html_response = xss_reflected_response.text().unwrap();
    if xss_reflected_html_response.contains(xss_params["name"]) {
        Ok(Some(format!("found in name parameter at this URL {}", full_url)))
    } else {
        Ok(None)
    }

    //println!("{}", xss_reflected_html_response);
}

fn get_usertoken(htmlresponse: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = Html::parse_document(htmlresponse);
    let selector = Selector::parse("input[name='user_token']").unwrap();

    if let Some(input) = document.select(&selector).next() {
        if let Some(token) = input.value().attr("value") {
            return Ok(token.to_string());
        }
    }
    Err("Could not find user_token".into())
}

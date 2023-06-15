use dotenvy::dotenv;
use std::env;
use std::io::BufRead;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // 環境変数からKINDLE_HIGHLIGHTS_URLを取得
    dotenv().expect("Failed to read .env file");
    let kindle_highlights_url = env::var("KINDLE_HIGHLIGHTS_URL").unwrap();
    let web_driver_url = env::var("WEB_DRIVER_URL").unwrap();

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(&web_driver_url, caps).await?;
    driver.goto(kindle_highlights_url).await?;

    let elem_form = driver.find(By::Css("form")).await?;
    let elem_email = elem_form.find(By::Css("input[type=email]")).await?;
    let elem_password = elem_form.find(By::Css("input[type=password]")).await?;
    let elem_submit = elem_form.find(By::Css("input[type=submit]")).await?;

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    print!("Enter your email:");
    let email = lines.next().unwrap().unwrap();
    print!("Enter your password:");
    let password = lines.next().unwrap().unwrap();

    elem_email.send_keys(email).await?;
    elem_password.send_keys(password).await?;
    elem_submit.click().await?;

    let elem_highlighted = driver.find(By::Css(".highlighted")).await?;
    let text = elem_highlighted.text().await?;
    println!("{}", text);

    elem_highlighted.click().await?;
    Ok(())
}

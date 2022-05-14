use anyhow::{anyhow, Result};
use reqwest::blocking::*;
use reqwest::cookie::Jar;

fn main() -> Result<()> {
    // build the client
    const DOMAIN: &str = "somesite.com"; // TODO replace
    const URL_PREFIX: &str = "https://";
    let jar = Jar::default();
    const COOKIE_NAME: &str = "PHPSESSID"; // TODO replace with cookie name
    let cookie_content = std::fs::read_to_string("cookie")?; // TODO put cookie body in a file named cookie
    let cookie_str = format!("{COOKIE_NAME}={cookie_content}; Domain={DOMAIN}");
    jar.add_cookie_str(&cookie_str, &format!("{URL_PREFIX}{DOMAIN}").parse()?);
    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(jar.into())
        .build()?;

    // build and send the request
    let path = "/some/path/"; // TODO replace
    let url = format!("{URL_PREFIX}{DOMAIN}{path}");
    let req = client.get(url).build()?;
    let body = client.execute(req)?.text()?;


    // parse the answer
    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse(r#"img[src*="chapter"]"#) // TODO css selector
        .map_err(|e| anyhow!("{e:?}"))?;
    let selected = document.select(&selector);
    for _element in selected {
        // TODO do something with the result
        // for example element.value().attr("src")
    }

    Ok(())
}

#![allow(unused)]

//use playwright_rs::prelude::*;

//use playwright_rs::Playwright;
//use playwright_rs::WaitUntil;

use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
//use std::fs::File;
use std::fs;
//use std::io::Write;

#[derive(Serialize, Deserialize)]
struct Payload {
    ps: String,
}

// fn expiry() -> Vec<String> {
//     vec!["09-2026"]
// }

pub async fn demo() -> Result<(), reqwest::Error> {
    let url = "https://live.euronext.com/nb/ajax/getPricesOptionsAjax/stock-options/YAR/DOSL";
    //let url = "https://live.euronext.com/nb/ajax/submitOptionsForm/stock-options/YAR/DOSL";

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36",
        ),
    );
    headers.insert(
        "X-Requested-With",
        header::HeaderValue::from_static("XMLHttpRequest"),
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/json"),
    );

    //"application/x-www-form-urlencoded; charset: UTF-8",

    let expirations = vec!["07-2026", "08-2026", "09-2026", "12-2026"];

    let payload = json!({ "ps": "999", "md[]": expirations});
    //let payload = "ps:999&md[]=07-2026&md[]=";

    let response = Client::new()
        .post(url)
        .headers(headers)
        .json(&payload)
        //.form(&payload) // Automatically URL-encodes data
        .send()
        .await?;

    if !response.status().is_success() {
        eprintln!("❌ Status: {}", response.status());
        let body = response.text().await?;
        eprintln!("❌ Body (likely HTML): {}", body);
        return Ok(());
    }

    let response_text = response.text().await?;
    println!("{}", response_text);

    fs::write("yar.json", response_text);

    Ok(())
}

/*
// Example of processing the response if you want to extract specific data
if let Some(simple_array) = json_value.get("simple").and_then(|v| v.as_array()) {
    for item in simple_array {
        if let Some(maturity) = item.get("maturityDate").and_then(|v| v.as_str()) {
            println!("Processing maturity date: {}", maturity);
            // Access 'data' field here if needed
        }
    }
}
*/

/*
pub async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Playwright
    let playwright = Playwright::launch().await?;

    // Launch Chromium in headless mode
    // let browser = playwright.chromium().launch().await?;
    let browser = playwright.webkit().launch().await?;

    // Create a new page
    let page = browser.new_page().await?;

    let nordnet_url = "https://www.nordnet.no/derivat/opsjoner/liste?currency=NOK&underlyingSymbol=YAR&expireDate=1797548400000";
    // Navigate to a URL
    page.goto(nordnet_url, None).await?;

    println!("Page Title: {}", page.title().await?);

    let html_content = page.content().await?;
    page.wait_for_load_state(Some(WaitUntil::DomContentLoaded))
        .await?;

    let mut file = File::create("example2.html")?;
    file.write_all(html_content.as_bytes())?;

    println!("HTML-innholdet ble lagret til 'example.html'!");

    // Cleanup
    browser.close().await?;
    Ok(())
}
*/

/*
import requests

# Setting headers fixes the 404/403 errors that occur in a normal browser address bar
headers = {
    "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36",
    "X-Requested-With": "XMLHttpRequest", # Identifies the script as a valid data call
}
url = "https://live.euronext.com/nb/ajax/getPricesOptionsAjax/stock-options/YAR/DOSL"

payload = {"ps": "999"} # Asks for all strike prices

# The .post() method satisfies the server's backend request requirement
response = requests.post(url, headers=headers, data=payload)

if response.status_code == 200:
    yar_json = response.json()
    #with open("yar.json", "w") as f:
    #    f.write(yar_json)
    print(yar_json)
else:
    print(f"Server response code: {response.status_code}")




https://live.euronext.com/en/markets/oslo/stock-options/list
https://live.euronext.com/en/product/stock-options/yar-dosl

https://stackoverflow.com/questions/77846337/webscraping-options-chain-from-euronext-for-all-expiry-dates-in-python

https://www.skatteetaten.no/bedrift-og-organisasjon/skatt/skattemelding-naringsdrivende/enk/
*/

/*

Løsning 1: Masker nettleseren (Viktigst)Når Playwright starter Chromium, legger den til en global variabel i
nettleseren kalt window.navigator.webdriver = true.

Sikkerhetssystemer sjekker denne umiddelbart. Du må fjerne dette flagget og legge til en ekte User-Agent.

Du gjør dette ved å sende inn BrowserContextOptions når du oppretter konteksten din:rustuse playwright_rs::Playwright;

use playwright_rs::browser::BrowserContextOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let playwright = Playwright::launch().await?;
    let browser = playwright.chromium().launch().await?;

    // Konfigurer konteksten til å etterligne en ekte bruker
    let context_options = BrowserContextOptions {
        // 1. Definer en helt vanlig User-Agent fra en vanlig Mac
        user_agent: Some("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36".to_string()),

        // 2. Sett skjermstørrelse så det ikke ser ut som en hodeløs server
        viewport: Some(playwright_rs::types::ViewportSize { width: 1440, height: 900 }),

        ..Default::default()
    };

    let context = browser.new_context(Some(context_options)).await?;
    let page = context.new_page().await?;

    // 3. Kjør et skript RETT før siden laster for å skjule at det er en bot
    page.add_init_script("Object.defineProperty(navigator, 'webdriver', {get: () => undefined})").await?;

    page.goto("https://nordnet.no", None).await?;

    Ok(())
}
Vær forsiktig når du bruker kodenLøsning 2:
Unngå "Headless"-deteksjonHvis du kjører koden hodeløst (headless: true), sender nettleseren ut subtile signaler
(som mangel på grafikkakselerasjon) som røper den.

For finanssider som Nordnet må du ofte tvinge den til å kjøre synlig,
eller bruke det nye new hodeløse flagget hvis biblioteket støtter det.

Prøv å tvinge frem et synlig vindu under utvikling for å se om det løser blokkeringen:rustuse playwright_rs::browser::LaunchOptions;

let launch_options = LaunchOptions {
    headless: Some(false), // Åpner et synlig Chrome-vindu på Mac-en din
    ..Default::default()
};

let browser = playwright.chromium().launch_with_options(launch_options).await?;

Vær forsiktig når du bruker kodenLøsning 3:
Vent på at "Cookie-samtykke" eller Modaler blokkererNoen ganger er ikke siden frosset av en bot-blokk, men av et usynlig eller delvis gjennomsiktig "Godta Cookies"-banner som ligger som et usynlig lag over hele skjermen. Dette hindrer deg i å klikke eller scrolle på elementene bak.Sjekk om du må klikke bort samtykket først:rust// Vent på at knappen for informasjonskapsler dukker opp, og klikk på den
if let Some(cookie_button) = page.locator("button:has-text('Godta')").first().await? {
    cookie_button.click(None).await?;
}

Vær forsiktig når du bruker kodenLøsning 4:
Hent opsjonsdataene direkte fra API-et i stedetHvis du bare skal ha tak i rådataene for opsjonspriser,
er det ofte mye lettere (og raskere) å bruke Playwright til å sniffe opp Nordnets interne API-kall
i stedet for å skrape HTML-siden manuelt.

Når du har løst blokkeringen med Løsning 1, kan du lytte etter JSON-responsene som Nordnet henter i bakgrunnen:rust// Lytt etter alle nettverksresponser som kommer inn på siden

page.on_response(|response| {
    if response.url().contains("api/v2/options") || response.url().contains("infors") {
        println!("Fant API-kall for opsjoner: {}", response.url());
        // Her kan du hente ut ren JSON med response.text() eller json() i stedet for å tolke HTML!
    }
});
Vær forsiktig når du bruker kodenDersom du tester å kjøre med headless:
  Some(false) (synlig vindu), dukker det opp en Cloudflare "Verify you are human" / Captcha-boks på skjermen din da?
*/

#![allow(unused)]

use playwright_rs::prelude::*;

async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Playwright
    let playwright = Playwright::initialize().await?;

    // Launch Chromium in headless mode
    let browser = playwright.chromium().launch().await?;

    // Create a new page
    let page = browser.new_page().await?;

    // Navigate to a URL
    page.goto("https://www.nordnet.no").await?;

    println!("Page Title: {}", page.title().await?);

    // Cleanup
    browser.close().await?;
    Ok(())
}

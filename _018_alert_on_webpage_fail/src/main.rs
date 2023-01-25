use notify_rust::Notification;
use headless_chrome::{Browser};
use anyhow::anyhow;

fn browse_wikipedia() -> Result<(), anyhow::Error> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to("https://www.wikipedia.org")?;
    tab.wait_for_element("input#searchInput")?.click()?;
    tab.type_str("WebKit")?.press_key("Enter")?;
    tab.wait_for_element("#firstHeading")?;

    if tab.get_url().ends_with("WebKitA") {
        return Ok(())
    }

    return Err(anyhow!("Missing attribute"))
}

fn main() {
    let message: &str;

    match browse_wikipedia() {
        Ok(_) => message = "Wikipedia is ok",
        Err(_) => message = "Wikipedia is down"
    }

    Notification::new()
        .summary("Wikipedia")
        .body(message)
        .icon("firefox")
        .show().expect("Could not show notification");
}

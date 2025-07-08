use {
    crate::error::{LlmWebError, Result},
    headless_chrome::{Browser, LaunchOptions, LaunchOptionsBuilder},
    std::ffi::OsStr,
};

pub struct LlmWebBrower {
    pub browser: Browser,
}

impl LlmWebBrower {
    pub async fn new() -> Result<LlmWebBrower> {
        let browser = stealthy_browser().await?;

        Ok(Self { browser })
    }

    pub async fn run(&self, url: &str) -> Result<String> {
        stealth_navigation(&self.browser, url).await
    }
}

async fn stealthy_browser() -> Result<Browser> {
    let opts = browser_launch_options().await?;

    let browser =
        Browser::new(opts).map_err(|e| LlmWebError::Browser(format!("Init Browser error: {e}")))?;

    Ok(browser)
}

async fn stealth_navigation(browser: &Browser, url: &str) -> Result<String> {
    let tab = browser
        .new_tab()
        .map_err(|e| LlmWebError::Browser(format!("Browser new_tab error: {e}")))?;

    tab.navigate_to(url)
        .map_err(|e| LlmWebError::Browser(format!("Browser navigate_to error: {e}")))?;

    tab.wait_until_navigated()
        .map_err(|e| LlmWebError::Browser(format!("Browser wait_until_navigated error: {e}")))?;

    let html = tab
        .get_content()
        .map_err(|e| LlmWebError::Browser(format!("Browser get_content error: {e}")))?;

    if is_js_blocked(&html) {
        return Err(LlmWebError::JsBlocked);
    }

    Ok(html)
}

pub fn is_js_blocked(html: &str) -> bool {
    html.contains("<h1>JavaScript is not available.</h1>")
        || html.contains("Please enable JavaScript")
}

async fn browser_launch_options<'a>() -> Result<LaunchOptions<'a>> {
    let v: Vec<&OsStr> = vec![
        OsStr::new("--disable-blink-features=AutomationControlled"),
        OsStr::new("--no-sandbox"),
        OsStr::new("--disable-web-security"),
        OsStr::new(
            "--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
        ),
        OsStr::new("--lang=en-US,en;q=0.9"),
        OsStr::new("--disable-dev-shm-usage"),
        OsStr::new("--disable-gpu"),
        OsStr::new("--disable-infobars"),
        OsStr::new("--no-first-run"),
        OsStr::new("--enable-automation=false"),
        OsStr::new("--enable-javascript=true"),
    ];

    LaunchOptionsBuilder::default()
        .headless(true)
        .window_size(Some((1200, 800)))
        .args(v)
        .build()
        .map_err(|e| LlmWebError::Browser(format!("{}", e)))
}

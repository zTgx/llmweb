use {
    crate::error::{LlmWebError, Result},
    headless_chrome::Browser,
    serde::{Serialize, de::DeserializeOwned},
    serde_json::json,
    std::fmt::Debug,
};

mod error;
mod models;

#[derive(Debug, Clone)]
pub enum LlmWebFormat {
    Json,
    Yaml,
    Text,
}

pub struct LlmWeb {
    client: models::LLMClient,
}

impl LlmWeb {
    pub fn new(name: &str) -> Self {
        Self {
            client: models::LLMClient::new(name),
        }
    }

    pub async fn completion<S, R>(&self, url: &str, scheme: S) -> Result<R>
    where
        S: Serialize + Debug,
        R: DeserializeOwned + Debug,
    {
        let browser = Browser::default()
            .map_err(|e| LlmWebError::Browser(format!("Init Browser error: {e}")))?;

        let tab = browser
            .new_tab()
            .map_err(|e| LlmWebError::Browser(format!("Browser new_tab error: {e}")))?;

        tab.navigate_to(url)
            .map_err(|e| LlmWebError::Browser(format!("Browser navigate_to error: {e}")))?;

        let user_content = tab
            .get_content()
            .map_err(|e| LlmWebError::Browser(format!("Browser get_content error: {e}")))?;

        let response = self.client.completion(&user_content, json!(scheme)).await?;

        let result: R = match serde_json::from_str(&response) {
            Ok(val) => val,
            Err(e) => {
                return Err(LlmWebError::SerdeJson(e));
            }
        };

        Ok(result)
    }
}

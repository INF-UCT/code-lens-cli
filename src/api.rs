use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum DurationMonths {
    One,
    Six,
    Twelve,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterTokenRequest {
    pub repository_url: String,
    pub expiration_months: DurationMonths,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub data: String,
}

pub struct ApiClient {
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        let base_url = std::env::var("CODELENS_SERVER_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        Self { base_url }
    }

    pub fn register_token(
        &self,
        repository_url: &str,
        expiration_months: DurationMonths,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/token/register", self.base_url);

        let request = RegisterTokenRequest {
            repository_url: repository_url.to_string(),
            expiration_months,
        };

        let response = ureq::post(&url)
            .set("Content-Type", "application/json")
            .send_json(&request)?;

        if response.status() == 200 {
            let body: TokenData = response.into_json()?;
            Ok(body.data)
        } else {
            let error_text = response
                .into_string()
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Server error: {}", error_text).into())
        }
    }

    pub fn refresh_token(
        &self,
        token: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/token/refresh/{}", self.base_url, token);

        let response = ureq::post(&url).call()?;

        if response.status() == 200 {
            let body: TokenData = response.into_json()?;
            Ok(body.data)
        } else {
            let error_text = response
                .into_string()
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Server error: {}", error_text).into())
        }
    }
}

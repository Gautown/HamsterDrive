//! HTTP客户端封装
use crate::utils::error::Result;

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| crate::utils::error::HamsterError::NetworkError(e.to_string()))?;
        
        Ok(Self { client })
    }

    pub async fn get(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(crate::utils::error::HamsterError::NetworkError(
                format!("GET请求失败: HTTP {}", response.status())
            ));
        }

        let text = response.text().await?;
        Ok(text)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("创建HttpClient失败")
    }
}

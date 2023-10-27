pub mod types;
use crate::types::GetAccountInfoRequest;
use crate::types::GetAccountInfoResponse;
use reqwest::header;

pub const HELIUS_API_DEVNET_URL: &str = "https://devnet.helius-rpc.com/?api-key=";
pub const HELIUS_API_MAINNET_URL: &str = "https://rpc.helius.xyz/?api-key=";

#[derive(Debug)]
pub struct HeliusClient {
    pub client: reqwest::Client,
    pub api_key: String,
}

impl HeliusClient {
    pub fn new(api_key: String) -> anyhow::Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-type",
            header::HeaderValue::from_static("application/json"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client, api_key })
    }

    pub async fn get_account_info(
        &self,
        public_key: String,
    ) -> anyhow::Result<GetAccountInfoResponse> {
        pub async fn get_account_info(
            client: reqwest::Client,
            url: String,
            public_key: String,
        ) -> anyhow::Result<GetAccountInfoResponse> {
            let request = GetAccountInfoRequest::new(public_key);
            let response = client
                .post(&url)
                .json(&request)
                .send()
                .await?
                .json()
                .await?;
            Ok(response)
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "dev")] {
                let url = format!("{}{}", HELIUS_API_DEVNET_URL, self.api_key);
                get_account_info(self.client.clone(), url, public_key).await
            } else {
                let url = format!("{}{}", HELIUS_API_MAINNET_URL, self.api_key);
                 get_account_info(self.client.clone(), url, public_key).await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HeliusClient;
    // test devnet
    // cargo test --features dev -- --nocapture
    #[tokio::test]
    async fn test_get_account_info() {
        let client = HeliusClient::new("".to_string()).unwrap();
        let response = client
            .get_account_info("ATrkCHG6PnkhVNaVz9tekg4je5cvZcLuZuF5UAxxEvyK".to_string())
            .await
            .unwrap();
        println!("{:#?}", response);
    }
}

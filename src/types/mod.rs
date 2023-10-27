use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetAccountInfoRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: (String, Encoding),
}

impl GetAccountInfoRequest {
    pub fn new(public_key: String) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "getAccountInfo".to_string(),
            params: (
                public_key,
                Encoding {
                    encoding: "base58".to_string(),
                },
            ),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Encoding {
    encoding: String,
}

#[test]
fn test_get_account_info_request() {
    let request =
        GetAccountInfoRequest::new("ATrkCHG6PnkhVNaVz9tekg4je5cvZcLuZuF5UAxxEvyK".to_string());

    let json = serde_json::to_string_pretty(&request).unwrap();
    println!("{}", json);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigObject {
    #[serde(rename = "Commitment")]
    pub commitment: String,
    #[serde(rename = "minContextSlot")]
    pub min_context_slot: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountInfoResponse {
    pub id: String,
    pub jsonrpc: String,
    pub result: Result,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub context: Context,
    pub value: Option<AccountInfoValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub slot: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfoValue {
    pub data: Vec<String>,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: u64,
    pub space: u64,
}

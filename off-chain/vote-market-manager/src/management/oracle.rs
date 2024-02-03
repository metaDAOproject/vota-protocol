use std::collections::HashMap;
use serde::Deserialize;
use crate::errors::VoteMarketManagerError;

#[derive(Debug, Deserialize)]
pub struct TokenPrice {
    usd: f64,
}
async fn fetch_token_prices(
    mut token_prices: HashMap<String, f64>,
    mint_addresses: Vec<&str>,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error + 'static>> {
    // API URL
    let api_url = format!(
        "https://api.coingecko.com/api/v3/simple/token_price/solana?contract_addresses={}&vs_currencies=usd",
        mint_addresses.join("%2C")
    );

    // Make the GET request

    let response = reqwest::get(&api_url).await?;
    let json_response: serde_json::Value = response.json().await?;
    for mint_address in mint_addresses {
        let price = json_response.get(mint_address);
        match price {
            Some(price) => {
                let price = price.get("usd").unwrap().as_f64().unwrap();
                token_prices.insert(mint_address.to_string(), price);
            }
            None => {
                return Err(VoteMarketManagerError::AddressNotFound.into());
            }
        }
    }
    Ok(token_prices.clone())
}
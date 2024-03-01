use crate::errors::VoteMarketManagerError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum KnownTokens {
    Uxd,
    Uxp,
    Msol,
    Blze,
    Sbr,
}

impl From<String> for KnownTokens {
    fn from(s: String) -> Self {
        match s.as_str() {
            "FLZYdni7sdh86U6eGUXK5epbzzt8Sc93PMPbMRTsYAqa" => KnownTokens::Uxd,
            "J5BWqabLwaFP3xPDGndRJdZPTUncQCRfostpDHh2eesb" => KnownTokens::Msol,
            "5VDD9VgkKBYMVsWekA9egrZTJsNs2cmgTm1YkPCCpz1U" => KnownTokens::Blze,
            "7kbnvuGBxxj8AG9qp8Scn56muWGaRaFqxg1FsRp3PaFT" => KnownTokens::Uxd,
            "UXPhBoR3qG4UCiGNJfV7MqhHyFqKN68g45GoYvAeL2M" => KnownTokens::Uxp,
            "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So" => KnownTokens::Msol,
            "BLZEEuZUBVqFhj8adcCFPJvPVCiCyVmh3hkJMrU8KuJA" => KnownTokens::Blze,
            "Saber2gLauYim4Mvftnrasomsv6NvAuncvMEZwcLpD1" => KnownTokens::Sbr,
            _ => panic!("Unknown token"),
        }
    }
}

impl From<Pubkey> for KnownTokens {
    fn from(key: Pubkey) -> Self {
        key.to_string().into()
    }
}

impl From<KnownTokens> for String {
    fn from(s: KnownTokens) -> Self {
        match s {
            KnownTokens::Uxd => "7kbnvuGBxxj8AG9qp8Scn56muWGaRaFqxg1FsRp3PaFT".to_string(),
            KnownTokens::Uxp => "UXPhBoR3qG4UCiGNJfV7MqhHyFqKN68g45GoYvAeL2M".to_string(),
            KnownTokens::Msol => "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So".to_string(),
            KnownTokens::Blze => "BLZEEuZUBVqFhj8adcCFPJvPVCiCyVmh3hkJMrU8KuJA".to_string(),
            KnownTokens::Sbr => "Saber2gLauYim4Mvftnrasomsv6NvAuncvMEZwcLpD1".to_string(),
        }
    }
}

pub fn fetch_token_prices(
    token_prices: &mut HashMap<KnownTokens, f64>,
    tokens: Vec<KnownTokens>,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mints: Vec<String> = tokens.iter().map(|x| (*x).into()).collect();
    let api_url = format!(
        "https://api.coingecko.com/api/v3/simple/token_price/solana?contract_addresses={}&vs_currencies=usd",
        mints.join("%2C")
    );
    let client = Client::new();
    let response = client.get(api_url).send()?;
    let json_response: serde_json::Value = response.json()?;
    for mint_address in mints {
        let price = json_response.get(mint_address.clone());
        match price {
            Some(price) => {
                let price = price.get("usd").unwrap().as_f64().unwrap();
                token_prices.insert(mint_address.into(), price);
            }
            None => {
                return Err(VoteMarketManagerError::AddressNotFound.into());
            }
        }
    }
    Ok(())
}

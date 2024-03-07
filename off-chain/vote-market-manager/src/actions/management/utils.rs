use solana_program::pubkey::Pubkey;
use std::error::Error;
use std::fs;
use std::str::FromStr;

pub fn get_relevant_gauges() -> Result<Vec<Pubkey>, Box<dyn Error>> {
    let gauges_file = fs::File::open("off-chain/vote-market-manager/info/gauges.json")?;
    let gauges: Vec<String> = serde_json::from_reader(gauges_file)?;
    let gauges = gauges
        .iter()
        .map(|g| Pubkey::from_str(g).unwrap())
        .collect::<Vec<Pubkey>>();
    Ok(gauges)
}

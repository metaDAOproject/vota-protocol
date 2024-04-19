use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;

pub fn short_address(address: &Pubkey) -> String {
    let mut short = String::new();
    let address = address.to_string();
    let len = address.len();
    short.push_str(&address[..4]);
    short.push_str("...");
    short.push_str(&address[len - 4..]);
    short
}

pub fn get_multiple_accounts(client: &RpcClient, keys : Vec<Pubkey>) -> Vec<Option<Account>> {
    // get 50 accounts at a time
    let mut accounts: Vec<Option<Account>> = Vec::new();
    for keys_chunk in keys.chunks(50) {
        let accounts_chunk = client.get_multiple_accounts(&keys_chunk).unwrap();
        accounts.extend(accounts_chunk);
    }
    accounts
}
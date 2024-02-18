use solana_program::pubkey::Pubkey;

pub fn short_address(address: &Pubkey) -> String {
    let mut short = String::new();
    let address = address.to_string();
    let len = address.len();
    short.push_str(&address[..4]);
    short.push_str("...");
    short.push_str(&address[len - 4..]);
    short
}
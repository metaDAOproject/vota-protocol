use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

pub fn set_maximum(
    anchor_client: &anchor_client::Client<&Keypair>,
    payer: &Keypair,
    gauge: Pubkey,
    config: Pubkey,
    epoch: u32,
    max_amount: u64,
) {
    let vote_buy = Pubkey::find_program_address(
        &[
            b"vote-buy".as_ref(),
            epoch.to_le_bytes().as_ref(),
            config.to_bytes().as_ref(),
            gauge.to_bytes().as_ref(),
        ],
        &vote_market::id(),
    )
    .0;

    let program = anchor_client.program(vote_market::id()).unwrap();
    let result = program
        .request()
        .signer(payer)
        .accounts(vote_market::accounts::SetMaxAmount {
            config,
            vote_buy,
            gauge,
            script_authority: payer.pubkey(),
        })
        .args(vote_market::instruction::SetMaxAmount { epoch, max_amount })
        .send();
    match result {
        Ok(sig) => {
            log::info!(target: "efficiency",
            sig=sig.to_string(),
            gauge=gauge.to_string(),
            epoch=epoch;
            "set maximum amount"
            );
            println!("Set maximum amount for {:?}: {:?}", vote_buy, sig);
        }
        Err(e) => {
            log::error!(target: "efficiency",
            error=e.to_string(),
            gauge=gauge.to_string(),
            epoch=epoch;
            "failed to set maximum amount");
            println!("Error setting maximum amount: {:?}", e);
        }
    }
}

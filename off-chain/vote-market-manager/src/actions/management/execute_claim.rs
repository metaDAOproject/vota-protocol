use crate::accounts::resolve::get_delegate;
use crate::actions::queries::escrows::get_delegated_escrows;
use crate::actions::queries::vote_buys::get_all_vote_buys;
use crate::actions::vote_market::claim::claim;
use anchor_client::Client;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

pub(crate) fn execute_claim(
    client: &RpcClient,
    anchor_client: &Client<&Keypair>,
    payer: &Keypair,
    config: Pubkey,
    epoch: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let delegate = get_delegate(&config);
    let escrows = get_delegated_escrows(client, &delegate);
    for vote_buy in get_all_vote_buys(epoch, &config) {
        for escrow in escrows.clone() {
            println!("claiming {:?} for {}", escrow.1.owner, vote_buy.gauge);
            claim(
                anchor_client,
                client,
                payer,
                escrow.1.owner,
                vote_buy.mint,
                escrow.0,
                config,
                vote_buy.gauge,
                epoch,
            )?;
        }
    }
    Ok(())
}

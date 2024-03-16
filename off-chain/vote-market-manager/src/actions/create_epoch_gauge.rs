use solana_client::rpc_client::RpcClient;
use solana_program::instruction::AccountMeta;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use crate::actions::retry_logic::retry_logic;

pub(crate) fn create_epoch_gauge(client: &RpcClient, payer: &Keypair, gauge: Pubkey, epoch: u32) {
    let (epoch_gauge, bump) = Pubkey::find_program_address(
        &[
            b"EpochGauge".as_ref(),
            &gauge.as_ref(),
            &epoch.to_le_bytes(),
        ],
        &gauge_state::id(),
    );
    let mut data: Vec<u8> =
        solana_program::hash::hash(b"global:create_epoch_gauge").to_bytes()[..8].to_vec();

    data.extend_from_slice(&bump.to_le_bytes());
    data.extend_from_slice(&epoch.to_le_bytes());
    let create_epoch_gauge_ix = solana_program::instruction::Instruction {
        program_id: gauge_state::id(),
        accounts: vec![
            AccountMeta::new(gauge, false),
            AccountMeta::new(epoch_gauge, false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data,
    };

    let mut ixs = vec![create_epoch_gauge_ix];
    let result = retry_logic(client, payer, &mut ixs);
    match result {
        Ok(sig) => {
            log::info!(
                target: "vote",
                sig = sig.to_string(),
                gauge = gauge.to_string(),
                epoch = epoch;
                "epoch gauge created"
            );
            println!("Created epoch gauge: {:?}", sig);
        }
        Err(e) => {
            log::error!(
                target: "vote",
                error = e.to_string(),
                gauge = gauge.to_string(),
                epoch = epoch;
                "failed to create epoch gauge"
            );
            println!("Error creating epoch gauge: {:?}", e);
        }
    }
}

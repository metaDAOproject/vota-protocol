use solana_client::rpc_client::RpcClient;
use solana_program::instruction::AccountMeta;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};


pub(crate) fn create_epoch_gauge(client: &RpcClient, payer: &Keypair, gauge: Pubkey, epoch: u32) {


    let (epoch_gauge, bump) = Pubkey::find_program_address(
        &[b"EpochGauge".as_ref(),
            &gauge.as_ref(),
            &epoch.to_le_bytes()],
        &gauge_state::id());
    let mut data: Vec<u8> =
        solana_program::hash::hash(b"global:create_epoch_gauge").to_bytes()[..8].to_vec();



    data.extend_from_slice(&bump.to_le_bytes());
    data.extend_from_slice(&epoch.to_le_bytes());
    // pub gauge: Account<'info, Gauge>,
    // pub epoch_gauge: Account<'info, EpochGauge>,
    // pub payer: Signer<'info>,
    // pub system_program: Program<'info, System>,
    let create_epoch_gauge_ix = solana_program::instruction::Instruction {
        program_id: gauge_state::id(),
        accounts: vec![
            AccountMeta::new(gauge, false),
            AccountMeta::new(epoch_gauge, false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(solana_program::system_program::id(), false)
        ],
        data,
    };

    let mut transaction =
        solana_sdk::transaction::Transaction::new_with_payer(&[create_epoch_gauge_ix],
                                                             Some(&payer.pubkey()));
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[payer], latest_blockhash);
    let result = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("created epoch gauge: {:?}", result);
}
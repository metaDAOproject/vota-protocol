import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, Program, web3} from "@coral-xyz/anchor";
import {VoteMarket} from "../target/types/vote_market";
import {expect} from "chai";
import BN from "bn.js";
import fs from 'fs';
import dotenv from 'dotenv';
import {setupTokens} from "./setup-tokens";
import {setupConfig} from "./test-setup";
import GAUGE_IDL from "../external-state/idls/gauge.json";
import {Gauge} from "../external-state/types/gauge";
import {GAUGE, GAUGE_PROGRAM_ID, GAUGEMEISTER, LOCKED_VOTER_PROGRAM_ID} from "./constants";
import {
    getAssociatedTokenAddressSync,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAccount
} from "@solana/spl-token";

dotenv.config();

describe("vote-market", () => {
    // Configure the client to use the local cluster.
    const rawKey = fs.readFileSync(process.env.KEY_PATH, 'utf-8');
    const payer = web3.Keypair.fromSecretKey(Uint8Array.from(JSON.parse(rawKey)));

    anchor.setProvider(AnchorProvider.env())
    const program = anchor.workspace.VoteMarket as Program<VoteMarket>;
    before( async () => {
        await program.provider.connection.requestAirdrop(payer.publicKey, 1000000000000)
        await new Promise(resolve => setTimeout(resolve, 1000));
    });
    const gaugeProgram = new Program(GAUGE_IDL as any, GAUGE_PROGRAM_ID) as Program<Gauge>;


    // it("Creates a config account", async () => {
    //   const {config, allowedMints,allowedMintList, scriptAuthority} = await setupConfig(program);
    //   const configAccount = await program.account.voteMarketConfig.fetch(config.publicKey);
    //   expect(configAccount.gaugemeister).to.eql(GAUGEMEISTER);
    //   expect(configAccount.scriptAuthority).to.eql(scriptAuthority);
    //   expect(configAccount.efficiencyRatio.eq(new BN(100))).to.be.true;
    //   const allowedMintsAccount = await program.account.allowedMints.fetch(allowedMints);
    //   expect(allowedMintsAccount.mints).to.eql(allowedMintList);
    // });
    // it("changes the admin account", async () => {
    //   const {config} = await setupConfig(program);
    //   const newAdmin = web3.Keypair.generate();
    //   //Should fail if the admin doesn't sign
    //   try {
    //
    //     await program.methods.updateAdmin(newAdmin.publicKey).accounts(
    //         {
    //           config: config.publicKey,
    //           admin: newAdmin .publicKey,
    //         }).signers([newAdmin]).rpc();
    //   } catch (e) {
    //     expect(e.message).to.contain("A has one constraint was violated");
    //   }
    //   await program.methods.updateAdmin(newAdmin.publicKey).accounts(
    //       {
    //         config: config.publicKey,
    //         admin: program.provider.publicKey,
    //       }).rpc();
    //   const configAccount = await program.account.voteMarketConfig.fetch(config.publicKey);
    //   expect(configAccount.admin).to.eql(newAdmin.publicKey);
    // });
    // it("changes the script authority account", async () => {
    //   const {config} = await setupConfig(program);
    //   const newScriptAuthority = web3.Keypair.generate();
    //   //Should fail if the admin doesn't sign
    //   try {
    //     await program.methods.updateScriptAuthority(newScriptAuthority.publicKey).accounts(
    //         {
    //           config: config.publicKey,
    //           admin: newScriptAuthority.publicKey,
    //         }).signers([newScriptAuthority]).rpc();
    //   } catch (e) {
    //     expect(e.message).to.contain("A has one constraint was violated");
    //   }
    //   await program.methods.updateScriptAuthority(newScriptAuthority.publicKey).accounts(
    //       {
    //         config: config.publicKey,
    //         admin: program.provider.publicKey,
    //       }).rpc();
    //   const configAccount = await program.account.voteMarketConfig.fetch(config.publicKey);
    //   expect(configAccount.scriptAuthority).to.eql(newScriptAuthority.publicKey);
    //
    // });
    // it("Updates the allowed mints list", async () => {
    //     const {config, allowedMints, allowedMintList} = await setupConfig(program);
    //       const newMint1 = web3.PublicKey.unique();
    //       const newMint2 = web3.PublicKey.unique();
    //
    //       let allowedMintsAccount = await program.provider.connection.getAccountInfo(allowedMints);
    //       expect(allowedMintsAccount!.data.length).to.eql(8 + 4 + 32 * 2);
    //       await program.methods.updateAllowedMints([...allowedMintList, newMint1, newMint2]).accounts(
    //           {
    //               config: config.publicKey,
    //               admin: program.provider.publicKey,
    //               allowedMints
    //           }).rpc();
    //       allowedMintsAccount = await program.provider.connection.getAccountInfo(allowedMints);
    //       expect(allowedMintsAccount!.data.length).to.eql(8 + 4 + 32 * 4);
    //       const allowedMintsData = await program.account.allowedMints.fetch(allowedMints);
    //       expect(allowedMintsData.mints).to.eql([...allowedMintList, newMint1, newMint2]);
    // });
    // it("Buyers can add payment", async () => {
    //     const {mint, ata} = await setupTokens(program, payer);
    //     const { config } = await setupConfig(program, [mint]);
    //     const gaugeMeisterData = await gaugeProgram.account.gaugemeister.fetch(GAUGEMEISTER);
    //     const epochBuffer = Buffer.alloc(4);
    //     epochBuffer.writeUInt32LE(gaugeMeisterData.currentRewardsEpoch);
    //     const [tokenBuy, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    //         [Buffer.from("token-buy"), epochBuffer, config.publicKey.toBuffer(), GAUGE.toBuffer()],
    //     program.programId);
    //     const tokenVault = getAssociatedTokenAddressSync(mint, tokenBuy, true);
    //     await program.methods.increaseVoteBuy(gaugeMeisterData.currentRewardsEpoch, new BN(1_000_000)).accounts(
    //         {
    //             buyer: program.provider.publicKey,
    //             buyerTokenAccount: ata,
    //             tokenVault,
    //             mint,
    //             config: config.publicKey,
    //             gaugemeister: GAUGEMEISTER,
    //             tokenBuy,
    //             gauge: GAUGE,
    //             tokenProgram: TOKEN_PROGRAM_ID,
    //             associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    //             systemProgram: web3.SystemProgram.programId,
    //         }).rpc({commitment: "confirmed"});
    //     const destinationTokenAccountData = await getAccount(program.provider.connection, tokenVault);
    //     expect(destinationTokenAccountData.amount === BigInt(1_000_000)).to.be.true;
    //     const tokenBuyData = await program.account.tokenBuy.fetch(tokenBuy);
    //     expect(tokenBuyData.amount.eq(new BN(1_000_000))).to.be.true;
    //     expect(tokenBuyData.mint).to.eql(destinationTokenAccountData.mint);
    //     expect(tokenBuyData.percentToUseBps.eq(new BN(0))).to.be.true;
    //     expect(tokenBuyData.rewardReceiver).to.eql(program.provider.publicKey);
    //     //Add more tokens
    //     await program.methods.increaseVoteBuy(gaugeMeisterData.currentRewardsEpoch, new BN(1_000_000)).accounts(
    //         {
    //             buyer: program.provider.publicKey,
    //             buyerTokenAccount: ata,
    //             tokenVault,
    //             mint,
    //             config: config.publicKey,
    //             gaugemeister: GAUGEMEISTER,
    //             tokenBuy,
    //             gauge: GAUGE,
    //             tokenProgram: TOKEN_PROGRAM_ID,
    //             associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    //             systemProgram: web3.SystemProgram.programId,
    //         }).rpc({commitment: "confirmed"});
    //         const tokenBuyDataMore = await program.account.tokenBuy.fetch(tokenBuy);
    //         expect(tokenBuyDataMore.amount.eq(new BN(2_000_000))).to.be.true;
    // });
    it("Vote sellers can withdraw vote payment", async () => {
        // Add payment
        const {mint, ata} = await setupTokens(program, payer);
        const { config } = await setupConfig(program, [mint]);
        const gaugeMeisterData = await gaugeProgram.account.gaugemeister.fetch(GAUGEMEISTER);
        const epochBuffer = Buffer.alloc(4);
        epochBuffer.writeUInt32LE(gaugeMeisterData.currentRewardsEpoch);
        const [tokenBuy, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("token-buy"), epochBuffer, config.publicKey.toBuffer(), GAUGE.toBuffer()],
            program.programId);
        const tokenVault = getAssociatedTokenAddressSync(mint, tokenBuy, true);
        await program.methods.increaseVoteBuy(gaugeMeisterData.currentRewardsEpoch, new BN(1_000_000)).accounts(
            {
                buyer: program.provider.publicKey,
                buyerTokenAccount: ata,
                tokenVault,
                mint,
                config: config.publicKey,
                gaugemeister: GAUGEMEISTER,
                tokenBuy,
                gauge: GAUGE,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: web3.SystemProgram.programId,
            }).rpc({commitment: "confirmed"});
        let [voteDelegate, voteDelegateBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("vote-delegate"), config.publicKey.toBuffer()],
            program.programId
        );

        let [escrow, escrowBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("Escrow"), gaugeMeisterData.locker.toBuffer(), program.provider.publicKey.toBuffer()],
            LOCKED_VOTER_PROGRAM_ID);

        let [gaugeVoter, gaugeVoterBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("GaugeVoter"), GAUGEMEISTER.toBuffer(), escrow.toBuffer()],
            GAUGE_PROGRAM_ID);

        let [gaugeVote, gaugeVoteBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("GaugeVote"), gaugeVoter.toBuffer(), GAUGE.toBuffer()],
            GAUGE_PROGRAM_ID);

        let [epochGaugeVoter, epochGaugeVoterBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("EpochGaugeVoter"), gaugeVoter.toBuffer(), Buffer.from(epochBuffer)],
            GAUGE_PROGRAM_ID);

        let [epochGaugeVote, epochGaugeVoteBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("EpochGaugeVote"), gaugeVote.toBuffer(), Buffer.from(epochBuffer)],
            GAUGE_PROGRAM_ID);

        let [epochGauge, epochGaugeBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("EpochGauge"), GAUGE.toBuffer(), Buffer.from(epochBuffer)],
            GAUGE_PROGRAM_ID);

        // Claim payment
        await program.methods.claimVotePayment(91).accounts({
            seller: program.provider.publicKey,
            sellerTokenAccount: ata,
            tokenVault,
            mint,
            config: config.publicKey,
            tokenBuy,
            voteDelegate,
            escrow,
            gaugemeister: GAUGEMEISTER,
            gaugeVoter,
            gaugeVote,
            epochGaugeVoter,
            epochGaugeVote,
            epochGauge,
            gauge: GAUGE,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: web3.SystemProgram.programId,
        }).rpc();
    // #[account(mut)]
    //     pub seller: Signer<'info>,
    // #[account(mut,
    //         associated_token::mint = mint,
    //         associated_token::authority = seller,
    //     )]
    //     pub seller_token_account: Account<'info, TokenAccount>,
    // #[account(mut,
    //         associated_token::mint = mint,
    //         associated_token::authority = token_buy,
    //     )]
    //     pub token_vault: Account<'info, TokenAccount>,
    //     pub mint: Account<'info, Mint>,
    // #[account(has_one = gaugemeister)]
    //     pub config: Account<'info, VoteMarketConfig>,
    // #[account(mut,
    //         seeds = [b"token-buy".as_ref(), epoch.to_le_bytes().as_ref(), config.key().as_ref(), gauge.key().as_ref()],
    //         bump,
    //         has_one = mint)]
    //     pub token_buy: Account<'info, TokenBuy>,
    // #[account(seeds = [b"vote-delegate", config.key().as_ref()], bump)]
    //     /// CHECK this is going to be a PDA signer to close the EpochGaugeVote. Verifying the seeds should be enough.
    //     pub vote_delegate: UncheckedAccount<'info>,
    // #[account(has_one = vote_delegate)]
    //     pub escrow: Account<'info, locked_voter_state::Escrow>,
    //     pub gaugemeister: Account<'info, gauge_state::Gaugemeister>,
    // #[account(has_one = gaugemeister, has_one = escrow)]
    //     pub gauge_voter: Account<'info, gauge_state::GaugeVoter>,
    // #[account(has_one = gauge_voter, has_one = gauge)]
    //     pub gauge_vote: Account<'info, gauge_state::GaugeVote>,
    // #[account(constraint = seller.key() == epoch_gauge_voter.gauge_voter)]
    //     pub epoch_gauge_voter: Account<'info, gauge_state::EpochGaugeVoter>,
    // #[account(has_one = gaugemeister)]
    //     pub gauge: Account<'info, gauge_state::Gauge>,
    //     pub epoch_gauge: Account<'info, gauge_state::EpochGauge>,
    // #[account(seeds = [b"EpochGaugeVote", gauge_vote.key().as_ref(), epoch.to_le_bytes().as_ref()], bump)]
    //     pub epoch_gauge_vote: Account<'info, gauge_state::EpochGaugeVote>,
    //     pub token_program: Program<'info, Token>,
    //     pub system_program: Program<'info, System>,
    // }


    });
    it("Buyers can withdraw rewards", async () => {
    });
    it("Sellers can withdraw rewards", async () => {
    });
});

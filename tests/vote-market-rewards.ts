import fs from "fs";
import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, Program, web3} from "@coral-xyz/anchor";
import {VoteMarket} from "../target/types/vote_market";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import GAUGE_IDL from "../external-state/idls/gauge.json";
import {GAUGE, GAUGE_PROGRAM_ID, GAUGEMEISTER, LOCKED_VOTER_PROGRAM_ID} from "./constants";
import {Gauge} from "../external-state/types/gauge";
import {setupTokens} from "./setup-tokens";
import {setupConfig} from "./test-setup";
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAccount,
    getAssociatedTokenAddressSync,
    TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import BN from "bn.js";
import dotenv from "dotenv";
import {expect} from "chai";
import * as crypto from "crypto";
import {getVoteAccounts} from "./common";

dotenv.config();


describe("vote market rewards phase", () => {
    // Configure the client to use the local cluster.
    const rawKey = fs.readFileSync(process.env.KEY_PATH, 'utf-8');
    const payer = web3.Keypair.fromSecretKey(Uint8Array.from(JSON.parse(rawKey)));

    const connection = new web3.Connection("http://127.0.0.1:8899", "confirmed");
    anchor.setProvider(new AnchorProvider(connection, new NodeWallet(payer), AnchorProvider.defaultOptions()));
    const program = anchor.workspace.VoteMarket as Program<VoteMarket>;
    before(async () => {
        await program.provider.connection.requestAirdrop(payer.publicKey, 1000000000000)
    });
    const gaugeProgram = new Program(GAUGE_IDL as any, GAUGE_PROGRAM_ID) as Program<Gauge>;
    it("Vote sellers can withdraw vote payment", async () => {
        // Add payment
        const {mint, ata} = await setupTokens(program, payer);

        const config = web3.Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync("tests/config.json", "utf-8"))));
        const {allowedMints} = await setupConfig(program, [mint], config);

        var gaugeMeisterData = await gaugeProgram.account.gaugemeister.fetch(GAUGEMEISTER);
        const epochBuffer = Buffer.alloc(4);
        epochBuffer.writeUInt32LE(gaugeMeisterData.currentRewardsEpoch + 1);
        const [voteBuy, _] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("vote-buy"), epochBuffer, config.publicKey.toBuffer(), GAUGE.toBuffer()],
            program.programId);
        const tokenVault = getAssociatedTokenAddressSync(mint, voteBuy, true);


        await program.methods.increaseVoteBuy(gaugeMeisterData.currentRewardsEpoch + 1, new BN(1_000_000)).accounts(
            {
                buyer: program.provider.publicKey,
                buyerTokenAccount: ata,
                tokenVault,
                mint,
                config: config.publicKey,
                gaugemeister: GAUGEMEISTER,
                voteBuy,
                gauge: GAUGE,
                allowedMints,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: web3.SystemProgram.programId,
            }).rpc({commitment: "confirmed"});
        let {
            voteDelegate,
            escrow,
            gaugeVoter,
            gaugeVote,
            epochGaugeVoter,
            epochGaugeVote,
            epochGauge
        } = getVoteAccounts(config, program, gaugeMeisterData, program.provider.publicKey);

        try {
            // Claim payment
            await program.methods.claimVotePayment(gaugeMeisterData.currentRewardsEpoch + 1).accounts({
                scriptAuthority: program.provider.publicKey,
                seller: program.provider.publicKey,
                sellerTokenAccount: ata,
                tokenVault,
                mint,
                config: config.publicKey,
                voteBuy,
                voteDelegate,
                escrow,
                gaugemeister: GAUGEMEISTER,
                gaugeVoter,
                gaugeVote,
                epochGaugeVoter,
                epochGaugeVote,
                epochGauge,
                gauge: GAUGE,
                gaugeProgram: GAUGE_PROGRAM_ID,
                lockedVoterProgram: LOCKED_VOTER_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: web3.SystemProgram.programId,
            }).rpc();
            expect.fail("Should have thrown an error when withdrawing vote payment before voting is complete");
        } catch (e) {
            console.log(e)
            expect(e.error.errorCode.code).to.equal("EpochVotingNotCompleted");
        }

        //get instruction discriminator
        var hash = crypto.createHash('sha256');
        hash.update(Buffer.from("global:trigger_next_epoch"));
        const discriminator = hash.digest().subarray(0, 8);
        const triggerNextEpochIx = new web3.TransactionInstruction({
            data: Buffer.from(discriminator),
            keys: [
                {pubkey: GAUGEMEISTER, isSigner: false, isWritable: true},
            ],
            programId: GAUGE_PROGRAM_ID,
        });
        const latestBlock = await program.provider.connection.getLatestBlockhash();
        const triggerNextEpochTx = new web3.Transaction({feePayer: payer.publicKey, ...latestBlock}).add(triggerNextEpochIx);
        triggerNextEpochTx.sign(payer);
        await program.provider.sendAndConfirm(triggerNextEpochTx, [], {commitment: "confirmed"});
        let sellerTokenAccountData = await getAccount(program.provider.connection, ata);
        expect(sellerTokenAccountData.amount === BigInt(999_000_000)).to.be.true;
        let tokenVaultData = await getAccount(program.provider.connection, tokenVault);
        expect(tokenVaultData.amount === BigInt(1_000_000)).to.be.true;

        const nonSellerPayer = web3.Keypair.generate();
        await program.provider.connection.requestAirdrop(nonSellerPayer.publicKey, 1000000000000)

        const updateScriptAuthSig = await program.methods.updateScriptAuthority(nonSellerPayer.publicKey).accounts(
            {
                config: config.publicKey,
                admin: program.provider.publicKey,
            }).rpc();
        const latestBlockhash = await program.provider.connection.getLatestBlockhash();
        await connection.confirmTransaction( {
                signature: updateScriptAuthSig,
                ...latestBlockhash
            }, "confirmed");
        const configData = await program.account.voteMarketConfig.fetch(config.publicKey);


        // Can't claim until a max amount is set
        await program.methods.setMaxAmount(gaugeMeisterData.currentRewardsEpoch + 1, new BN(1_000_000)).accounts( {
                config: config.publicKey,
                gauge: GAUGE,
                voteBuy,
                scriptAuthority: nonSellerPayer.publicKey
            }
        ).signers([nonSellerPayer]).rpc();

        const tx = await program.methods.claimVotePayment(gaugeMeisterData.currentRewardsEpoch + 1).accounts({
            scriptAuthority: nonSellerPayer.publicKey,
            seller: program.provider.publicKey,
            sellerTokenAccount: ata,
            tokenVault,
            mint,
            config: config.publicKey,
            voteBuy,
            voteDelegate,
            escrow,
            gaugemeister: GAUGEMEISTER,
            gaugeVoter,
            gaugeVote,
            epochGaugeVoter,
            epochGaugeVote,
            epochGauge,
            gauge: GAUGE,
            gaugeProgram: GAUGE_PROGRAM_ID,
            lockedVoterProgram: LOCKED_VOTER_PROGRAM_ID,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: web3.SystemProgram.programId,
        }).signers([nonSellerPayer]).rpc();
        // }).transaction();
        // tx.recentBlockhash = latestBlockhash.blockhash;
        // tx.lastValidBlockHeight = latestBlockhash.lastValidBlockHeight;
        // tx.feePayer = nonSellerPayer.publicKey;
        // tx.sign(nonSellerPayer);
        // const sig = await program.provider.connection.sendRawTransaction(tx.serialize());
        // program.provider.connection.confirmTransaction({
        //     signature: sig,
        //     ...latestBlockhash
        // }, "confirmed");
//        await new Promise((resolve) => setTimeout(resolve, 10000));
        sellerTokenAccountData = await getAccount(program.provider.connection, ata);
        const expectedRewards = BigInt(18514);

        expect(sellerTokenAccountData.amount).to.equal(BigInt(999_000_000) + expectedRewards);
        tokenVaultData = await getAccount(program.provider.connection, tokenVault);
        expect(tokenVaultData.amount).to.equal(BigInt(1_000_000) - expectedRewards);
        const voteDelegateBalance = await program.provider.connection.getBalance(voteDelegate);
        const expectedGaugeVoteRent = await program.provider.connection.getMinimumBalanceForRentExemption(16)
        expect(voteDelegateBalance).to.equal(expectedGaugeVoteRent);

        //Should not be able to claim again
        try {
            await program.methods.claimVotePayment(gaugeMeisterData.currentRewardsEpoch + 1).accounts({
                scriptAuthority: nonSellerPayer.publicKey,
                seller: program.provider.publicKey,
                sellerTokenAccount: ata,
                tokenVault,
                mint,
                config: config.publicKey,
                voteBuy,
                voteDelegate,
                escrow,
                gaugemeister: GAUGEMEISTER,
                gaugeVoter,
                gaugeVote,
                epochGaugeVoter,
                epochGaugeVote,
                epochGauge,
                gauge: GAUGE,
                gaugeProgram: GAUGE_PROGRAM_ID,
                lockedVoterProgram: LOCKED_VOTER_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: web3.SystemProgram.programId,
            }).signers([nonSellerPayer]).rpc({commitment: "confirmed"});
            expect.fail("Claimed vote payment twice");
        } catch (e) {
            expect(e.error.errorCode.code).to.equal("AccountNotInitialized");
        }
    });
    it("Buyers can withdraw rewards", async () => {
    });
    it("Sellers can withdraw rewards", async () => {
    });
});
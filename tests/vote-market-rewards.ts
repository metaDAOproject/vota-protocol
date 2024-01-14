import fs from "fs";
import {AnchorProvider, Program, web3} from "@coral-xyz/anchor";
import {VoteMarket} from "../target/types/vote_market";
import * as anchor from "@coral-xyz/anchor";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import GAUGE_IDL from "../external-state/idls/gauge.json";
import {GAUGE, GAUGE_PROGRAM_ID, GAUGEMEISTER, LOCKED_VOTER_PROGRAM_ID} from "./constants";
import {Gauge} from "../external-state/types/gauge";
import {setupTokens} from "./setup-tokens";
import {setupConfig} from "./test-setup";
import crypto from "crypto";
import {ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID} from "@solana/spl-token";
import BN from "bn.js";

describe("vote market rewards phase", () => {
    // Configure the client to use the local cluster.
    const rawKey = fs.readFileSync(process.env.KEY_PATH, 'utf-8');
    const payer = web3.Keypair.fromSecretKey(Uint8Array.from(JSON.parse(rawKey)));

    const connection = new web3.Connection("http://127.0.0.1:8899", "confirmed");
    anchor.setProvider(new AnchorProvider(connection, new NodeWallet(payer), AnchorProvider.defaultOptions()));
    const program = anchor.workspace.VoteMarket as Program<VoteMarket>;
    before( async () => {
        await program.provider.connection.requestAirdrop(payer.publicKey, 1000000000000)
        await new Promise(resolve => setTimeout(resolve, 1000));
    });
    const gaugeProgram = new Program(GAUGE_IDL as any, GAUGE_PROGRAM_ID) as Program<Gauge>;
    it("Vote sellers can withdraw vote payment", async () => {
        // Add payment
        const {mint, ata} = await setupTokens(program, payer);

        const config = web3.Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync("tests/config.json", "utf-8"))));
        await setupConfig(program, [mint], config);

        var gaugeMeisterData = await gaugeProgram.account.gaugemeister.fetch(GAUGEMEISTER);
        console.log("Current epoch", gaugeMeisterData.currentRewardsEpoch);

        const epochBuffer = Buffer.alloc(4);
        epochBuffer.writeUInt32LE(gaugeMeisterData.currentRewardsEpoch + 1);
        const [tokenBuy, _] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("token-buy"), epochBuffer, config.publicKey.toBuffer(), GAUGE.toBuffer()],
            program.programId);
        const epochBuffer2 = Buffer.alloc(4);
        epochBuffer2.writeUInt32LE(gaugeMeisterData.currentRewardsEpoch + 2);
        const [tokenBuy2, _2] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("token-buy"), epochBuffer2, config.publicKey.toBuffer(), GAUGE.toBuffer()],
            program.programId);
        console.log("tokenBuy", tokenBuy.toBase58());
        console.log("tokenBuy2", tokenBuy2.toBase58());
        const tokenVault = getAssociatedTokenAddressSync(mint, tokenBuy, true);
        await program.methods.increaseVoteBuy(gaugeMeisterData.currentRewardsEpoch + 1, new BN(1_000_000)).accounts(
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
        const result = await program.provider.sendAndConfirm(triggerNextEpochTx, [], {commitment: "confirmed"});
        console.log("Trigger next epoch result", result);
        var gaugeMeisterData2 = await gaugeProgram.account.gaugemeister.fetch(GAUGEMEISTER);
        console.log("Current epoch", gaugeMeisterData2.currentRewardsEpoch);

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
        console.log("Locker is", gaugeMeisterData.locker.toBase58());
        console.log("Owner is", program.provider.publicKey.toBase58());
        console.log("Escrow is", escrow.toBase58());
        console.log("GaugeVoter is", gaugeVoter.toBase58());
        console.log("GaugeVote is", gaugeVote.toBase58());

        let [epochGaugeVoter, epochGaugeVoterBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("EpochGaugeVoter"), gaugeVoter.toBuffer(), Buffer.from(epochBuffer)],
            GAUGE_PROGRAM_ID);
        console.log("epochGaugeVpoter", epochGaugeVoter.toBase58());
        let [epochGaugeVoter2, epochGaugeVoterBump2] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("EpochGaugeVoter"), gaugeVoter.toBuffer(), Buffer.from(epochBuffer2)],
            GAUGE_PROGRAM_ID);
        console.log("epochGaugeVpoter 2", epochGaugeVoter2.toBase58());

        let [epochGaugeVote, epochGaugeVoteBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("EpochGaugeVote"), gaugeVote.toBuffer(), Buffer.from(epochBuffer)],
            GAUGE_PROGRAM_ID);

        let [epochGauge, epochGaugeBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("EpochGauge"), GAUGE.toBuffer(), Buffer.from(epochBuffer)],
            GAUGE_PROGRAM_ID);

        // Claim payment
        await program.methods.claimVotePayment(gaugeMeisterData.currentRewardsEpoch + 1).accounts({
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
    });
    it("Buyers can withdraw rewards", async () => {
    });
    it("Sellers can withdraw rewards", async () => {
    });
});
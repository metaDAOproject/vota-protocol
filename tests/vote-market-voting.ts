import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, Program, web3} from "@coral-xyz/anchor";
import {VoteMarket} from "../target/types/vote_market";
import BN from "bn.js";
import fs from 'fs';
import dotenv from 'dotenv';
import {setupTokens} from "./setup-tokens";
import {setupConfig} from "./test-setup";
import GAUGE_IDL from "../external-state/idls/gauge.json";
import {Gauge} from "../external-state/types/gauge";
import {GAUGE, GAUGE_PROGRAM_ID, GAUGEMEISTER} from "./constants";
import {
    ASSOCIATED_TOKEN_PROGRAM_ID, createAssociatedTokenAccount,
    getAccount,
    getAssociatedTokenAddressSync, mintTo,
    TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import {expect} from "chai";
import {exec} from "child_process";
import {getVoteAccounts} from "./common";

dotenv.config();

describe("vote market voting phase", () => {
    // Configure the client to use the local cluster.
    const rawKey = fs.readFileSync(process.env.KEY_PATH, 'utf-8');
    const payer = web3.Keypair.fromSecretKey(Uint8Array.from(JSON.parse(rawKey)));
    const payer2RawKey = fs.readFileSync(process.env.KEY_PATH2, 'utf-8');
    // Payer owning the escrow that hasn't voted yet
    const payer2 = web3.Keypair.fromSecretKey(Uint8Array.from(JSON.parse(payer2RawKey)));
    const connection = new web3.Connection("http://127.0.0.1:8899", "confirmed");
    connection.requestAirdrop(payer2.publicKey, 1000000000000);
    anchor.setProvider(new AnchorProvider(connection, new NodeWallet(payer), {
        ...AnchorProvider.defaultOptions(),
        commitment: "confirmed"
    }));
    const program = anchor.workspace.VoteMarket as Program<VoteMarket>;
    before(async () => {
        await program.provider.connection.requestAirdrop(payer.publicKey, 1000000000000)
        await new Promise(resolve => setTimeout(resolve, 1000));
    });
    const gaugeProgram = new Program(GAUGE_IDL as any, GAUGE_PROGRAM_ID) as Program<Gauge>;
    it("Creates a config account", async () => {
        const {config, allowedMints, allowedMintList, scriptAuthority} = await setupConfig(program);
        const configAccount = await program.account.voteMarketConfig.fetch(config.publicKey);
        expect(configAccount.gaugemeister).to.eql(GAUGEMEISTER);
        expect(configAccount.scriptAuthority).to.eql(scriptAuthority);
        expect(configAccount.efficiencyRatio.eq(new BN(100))).to.be.true;
        const allowedMintsAccount = await program.account.allowedMints.fetch(allowedMints);
        expect(allowedMintsAccount.mints).to.eql(allowedMintList);
    });
    it("Changes the admin account", async () => {
        const {config} = await setupConfig(program);
        const newAdmin = web3.Keypair.generate();
        //Should fail if the admin doesn't sign
        try {

            await program.methods.updateAdmin(newAdmin.publicKey).accounts(
                {
                    config: config.publicKey,
                    admin: newAdmin.publicKey,
                }).signers([newAdmin]).rpc();
        } catch (e) {
            expect(e.message).to.contain("A has one constraint was violated");
        }
        await program.methods.updateAdmin(newAdmin.publicKey).accounts(
            {
                config: config.publicKey,
                admin: program.provider.publicKey,
            }).rpc();
        const configAccount = await program.account.voteMarketConfig.fetch(config.publicKey);
        expect(configAccount.admin).to.eql(newAdmin.publicKey);
    });
    it("Changes the script authority account", async () => {
        const {config} = await setupConfig(program);
        const newScriptAuthority = web3.Keypair.generate();
        //Should fail if the admin doesn't sign
        try {
            await program.methods.updateScriptAuthority(newScriptAuthority.publicKey).accounts(
                {
                    config: config.publicKey,
                    admin: newScriptAuthority.publicKey,
                }).signers([newScriptAuthority]).rpc();
            expect.fail("Needs to fail without correct signer");
        } catch (e) {
            expect(e.message).to.contain("A has one constraint was violated");
        }
        await program.methods.updateScriptAuthority(newScriptAuthority.publicKey).accounts(
            {
                config: config.publicKey,
                admin: program.provider.publicKey,
            }).rpc();
        const configAccount = await program.account.voteMarketConfig.fetch(config.publicKey);
        expect(configAccount.scriptAuthority).to.eql(newScriptAuthority.publicKey);

    });
    it("Updates the allowed mints list", async () => {
        const {config, allowedMints, allowedMintList} = await setupConfig(program);
        const newMint1 = web3.PublicKey.unique();
        const newMint2 = web3.PublicKey.unique();

        let allowedMintsAccount = await program.provider.connection.getAccountInfo(allowedMints);
        expect(allowedMintsAccount!.data.length).to.eql(8 + 4 + 32 * 2);
        await program.methods.updateAllowedMints([...allowedMintList, newMint1, newMint2]).accounts(
            {
                config: config.publicKey,
                admin: program.provider.publicKey,
                allowedMints
            }).rpc();
        allowedMintsAccount = await program.provider.connection.getAccountInfo(allowedMints);
        expect(allowedMintsAccount!.data.length).to.eql(8 + 4 + 32 * 4);
        const allowedMintsData = await program.account.allowedMints.fetch(allowedMints);
        expect(allowedMintsData.mints).to.eql([...allowedMintList, newMint1, newMint2]);
    });
    it("Buyers can add payment", async () => {
        const {mint, ata, mintAuth} = await setupTokens(program, payer);
        const {config, allowedMints} = await setupConfig(program, [mint]);
        const gaugeMeisterData = await gaugeProgram.account.gaugemeister.fetch(GAUGEMEISTER);
        const epochBuffer = Buffer.alloc(4);
        epochBuffer.writeUInt32LE(gaugeMeisterData.currentRewardsEpoch + 1);
        const [voteBuy, bump] = anchor.web3.PublicKey.findProgramAddressSync(
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
                tokenProgram: TOKEN_PROGRAM_ID,
                allowedMints,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: web3.SystemProgram.programId,
            }).rpc({commitment: "confirmed"});
        const destinationTokenAccountData = await getAccount(program.provider.connection, tokenVault);
        expect(destinationTokenAccountData.amount === BigInt(1_000_000)).to.be.true;
        const voteBuyData = await program.account.voteBuy.fetch(voteBuy);
        expect(voteBuyData.amount.eq(new BN(1_000_000))).to.be.true;
        expect(voteBuyData.mint).to.eql(destinationTokenAccountData.mint);
        expect(voteBuyData.maxAmount.eq(new BN(0))).to.be.true;
        expect(voteBuyData.rewardReceiver).to.eql(program.provider.publicKey);
        //Add more tokens
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
                tokenProgram: TOKEN_PROGRAM_ID,
                allowedMints,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: web3.SystemProgram.programId,
            }).rpc({commitment: "confirmed", skipPreflight: true});
        const voteBuyDataMore = await program.account.voteBuy.fetch(voteBuy);
        expect(voteBuyDataMore.amount.eq(new BN(2_000_000))).to.be.true;
        // Try invalid buyer
        const invalid_buyer = web3.Keypair.generate();

        const invalid_ata = await createAssociatedTokenAccount(
            program.provider.connection,
            payer,
            mint,
            invalid_buyer.publicKey
        );
        await mintTo(
            program.provider.connection,
            payer,
            mint,
            invalid_ata,
            mintAuth,
            BigInt(1000000000),
            []
        );

        try {
            await program.methods.increaseVoteBuy(gaugeMeisterData.currentRewardsEpoch + 1, new BN(1_000_000)).accounts(
                {
                    buyer: invalid_buyer.publicKey,
                    buyerTokenAccount: invalid_ata,
                    tokenVault,
                    mint,
                    config: config.publicKey,
                    gaugemeister: GAUGEMEISTER,
                    voteBuy,
                    gauge: GAUGE,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    allowedMints,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    systemProgram: web3.SystemProgram.programId,
                }).signers([invalid_buyer]).rpc({commitment: "confirmed"});
            expect.fail("Invalid buyer");
        } catch (e) {
            expect(e.error.errorCode.code).to.equal("InvalidBuyer");
        }

        // Check if the max amount can be set successfully
        await program.methods.setMaxAmount(gaugeMeisterData.currentRewardsEpoch + 1, new BN(1000)).accounts( {
                config: config.publicKey,
                gauge: GAUGE,
                voteBuy,
                scriptAuthority: program.provider.publicKey,
            }
        ).rpc();
        const voteBuyDataMax = await program.account.voteBuy.fetch(voteBuy);
        expect(voteBuyDataMax.maxAmount.eq(new BN(1000))).to.be.true;
    });
    it("Buyers must use mint on allow list", async () => {
        const {mint, ata} = await setupTokens(program, payer);
        const {config, allowedMints} = await setupConfig(program);
        const gaugeMeisterData = await gaugeProgram.account.gaugemeister.fetch(GAUGEMEISTER);
        const epochBuffer = Buffer.alloc(4);
        epochBuffer.writeUInt32LE(gaugeMeisterData.currentRewardsEpoch + 1);
        const [voteBuy, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("vote-buy"), epochBuffer, config.publicKey.toBuffer(), GAUGE.toBuffer()],
            program.programId);
        const tokenVault = getAssociatedTokenAddressSync(mint, voteBuy, true);
        try {
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
                    tokenProgram: TOKEN_PROGRAM_ID,
                    allowedMints,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    systemProgram: web3.SystemProgram.programId,
                }).rpc({commitment: "confirmed"});
            expect.fail("Buyer used a mint not on the allow list");
        } catch (e) {
            expect(e.error.errorCode.code).to.equal("InvalidMint");
        }

    });
    it("Can vote on behalf of the user", async () => {
        const {mint, ata, mintAuth} = await setupTokens(program, payer);

        const config = web3.Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync("tests/config2.json", "utf-8"))));
        await setupConfig(program, [mint], config);
        // Get an account with SBR deposited
        const gaugeMeisterData = await gaugeProgram.account.gaugemeister.fetch(GAUGEMEISTER);
        const delegate = web3.PublicKey.findProgramAddressSync([
            Buffer.from("vote-delegate"),
            config.publicKey.toBuffer(),
        ], program.programId)[0];
        program.provider.connection.requestAirdrop(delegate, 1000000000000);

        await program.methods.updateScriptAuthority(payer2.publicKey).accounts(
            {
                config: config.publicKey,
                admin: program.provider.publicKey,
            }).rpc();

        let {
            escrow,
            gaugeVoter,
            gaugeVote,
        } = getVoteAccounts(config, program, gaugeMeisterData, payer2.publicKey);
        const voteAccount = await gaugeProgram.account.gaugeVote.fetch(gaugeVote);
        expect(voteAccount.weight).to.equal(0);
        const builder = program.methods.vote(100).accounts({
            scriptAuthority: payer2.publicKey,
            config: config.publicKey,
            gaugemeister: GAUGEMEISTER,
            gauge: GAUGE,
            gaugeVoter,
            gaugeVote,
            escrow,
            voteDelegate: delegate,
            gaugeProgram: GAUGE_PROGRAM_ID,
        });
        const tx = await builder.transaction();

        const sig = await program.provider.connection.sendTransaction(tx, [payer2], {
        });
        const latestBlockhash = await program.provider.connection.getLatestBlockhash();
        await program.provider.connection.confirmTransaction({
            signature: sig,
            ...latestBlockhash
        }, "confirmed");
        const voteAccountAfter = await gaugeProgram.account.gaugeVote.fetch(gaugeVote);
        await new Promise(resolve => setTimeout(resolve, 10000));
        expect(voteAccountAfter.weight).to.equal(100);
    })
});

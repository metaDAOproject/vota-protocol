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
import {GAUGE, GAUGE_PROGRAM_ID, GAUGEMEISTER} from "./constants";
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


    it("Creates a config account", async () => {
      const {config, allowedMints,allowedMintList, scriptAuthority} = await setupConfig(program);
      const configAccount = await program.account.voteMarketConfig.fetch(config.publicKey);
      expect(configAccount.gaugemeister).to.eql(GAUGEMEISTER);
      expect(configAccount.scriptAuthority).to.eql(scriptAuthority);
      expect(configAccount.efficiencyRatio.eq(new BN(100))).to.be.true;
      const allowedMintsAccount = await program.account.allowedMints.fetch(allowedMints);
      expect(allowedMintsAccount.mints).to.eql(allowedMintList);
    });
    it("changes the admin account", async () => {
      const {config} = await setupConfig(program);
      const newAdmin = web3.Keypair.generate();
      //Should fail if the admin doesn't sign
      try {

        await program.methods.updateAdmin(newAdmin.publicKey).accounts(
            {
              config: config.publicKey,
              admin: newAdmin .publicKey,
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
    it("changes the script authority account", async () => {
      const {config} = await setupConfig(program);
      const newScriptAuthority = web3.Keypair.generate();
      //Should fail if the admin doesn't sign
      try {
        await program.methods.updateScriptAuthority(newScriptAuthority.publicKey).accounts(
            {
              config: config.publicKey,
              admin: newScriptAuthority.publicKey,
            }).signers([newScriptAuthority]).rpc();
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
        const {mint, ata} = await setupTokens(program, payer);
        const { config } = await setupConfig(program, [mint]);
        const gaugeMeisterData = await gaugeProgram.account.gaugemeister.fetch(GAUGEMEISTER);
        const epochBuffer = Buffer.alloc(4);
        epochBuffer.writeUInt32LE(gaugeMeisterData.currentRewardsEpoch);
        const [tokenBuy, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("token-buy"), epochBuffer, config.publicKey.toBuffer(), GAUGE.toBuffer()],
        program.programId);
        const destinationTokenAccount = getAssociatedTokenAddressSync(mint, tokenBuy, true);
        await program.methods.increaseVoteBuy(gaugeMeisterData.currentRewardsEpoch, new BN(1_000_000)).accounts(
            {
                buyer: program.provider.publicKey,
                buyerTokenAccount: ata,
                destinationTokenAccount,
                mint,
                config: config.publicKey,
                gaugemeister: GAUGEMEISTER,
                tokenBuy,
                gauge: GAUGE,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: web3.SystemProgram.programId,
            }).rpc({commitment: "confirmed"});
        const destinationTokenAccountData = await getAccount(program.provider.connection, destinationTokenAccount);
        expect(destinationTokenAccountData.amount === BigInt(1_000_000)).to.be.true;
        const tokenBuyData = await program.account.tokenBuy.fetch(tokenBuy);
        expect(tokenBuyData.amount.eq(new BN(1_000_000))).to.be.true;
        expect(tokenBuyData.mint).to.eql(destinationTokenAccountData.mint);
        expect(tokenBuyData.percentToUseBps.eq(new BN(0))).to.be.true;
        expect(tokenBuyData.rewardReceiver).to.eql(program.provider.publicKey);
        //Add more tokens
        await program.methods.increaseVoteBuy(gaugeMeisterData.currentRewardsEpoch, new BN(1_000_000)).accounts(
            {
                buyer: program.provider.publicKey,
                buyerTokenAccount: ata,
                destinationTokenAccount,
                mint,
                config: config.publicKey,
                gaugemeister: GAUGEMEISTER,
                tokenBuy,
                gauge: GAUGE,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: web3.SystemProgram.programId,
            }).rpc({commitment: "confirmed"});
            const tokenBuyDataMore = await program.account.tokenBuy.fetch(tokenBuy);
            expect(tokenBuyDataMore.amount.eq(new BN(2_000_000))).to.be.true;
    });
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
        const destinationTokenAccount = getAssociatedTokenAddressSync(mint, tokenBuy, true);
        await program.methods.increaseVoteBuy(gaugeMeisterData.currentRewardsEpoch, new BN(1_000_000)).accounts(
            {
                buyer: program.provider.publicKey,
                buyerTokenAccount: ata,
                destinationTokenAccount,
                mint,
                config: config.publicKey,
                gaugemeister: GAUGEMEISTER,
                tokenBuy,
                gauge: GAUGE,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: web3.SystemProgram.programId,
            }).rpc({commitment: "confirmed"});
        // Claim payment
        await program.methods.claimVotePayment().rpc();


    });
    it("Buyers can withdraw rewards", async () => {
    });
    it("Sellers can withdraw rewards", async () => {
    });
});

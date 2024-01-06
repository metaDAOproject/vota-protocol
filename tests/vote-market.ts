import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, Program, web3} from "@coral-xyz/anchor";
import {VoteMarket} from "../target/types/vote_market";
import {expect} from "chai";
import BN, {min} from "bn.js";
import {setupConfig} from "./test-setup";
import {createMint, MintLayout} from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import fs from 'fs';
import dotenv from 'dotenv';

dotenv.config();

describe("vote-market", () => {
  // Configure the client to use the local cluster.
  const rawKey = fs.readFileSync(process.env.KEY_PATH, 'utf-8');
  const payer =  web3.Keypair.fromSecretKey(Uint8Array.from(JSON.parse(rawKey)));
  anchor.setProvider(AnchorProvider.env())


  const program = anchor.workspace.VoteMarket as Program<VoteMarket>;

  // it("Creates a config account", async () => {
  //   const {config, allowedMints, mint1, mint2, gaugemeister, scriptAuthority, tx} = await setupConfig(program);
  //   const configAccount = await program.account.voteMarketConfig.fetch(config.publicKey);
  //   expect(configAccount.gaugemeister).to.eql(gaugemeister);
  //   expect(configAccount.scriptAuthority).to.eql(scriptAuthority);
  //   expect(configAccount.efficiencyRatio.eq(new BN(100))).to.be.true;
  //   const allowedMintsAccount = await program.account.allowedMints.fetch(allowedMints);
  //   expect(allowedMintsAccount.mints).to.eql([mint1, mint2]);
  //   console.log(tx);
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
  //     const {config, allowedMints, mint1, mint2} = await setupConfig(program);
  //       const newMint1 = web3.PublicKey.unique();
  //       const newMint2 = web3.PublicKey.unique();
  //
  //       let allowedMintsAccount = await program.provider.connection.getAccountInfo(allowedMints);
  //       expect(allowedMintsAccount!.data.length).to.eql(8 + 4 + 32 * 2);
  //       await program.methods.updateAllowedMints([mint1, mint2, newMint1, newMint2]).accounts(
  //           {
  //               config: config.publicKey,
  //               admin: program.provider.publicKey,
  //               allowedMints
  //           }).rpc();
  //       allowedMintsAccount = await program.provider.connection.getAccountInfo(allowedMints);
  //       expect(allowedMintsAccount!.data.length).to.eql(8 + 4 + 32 * 4);
  //       const allowedMintsData = await program.account.allowedMints.fetch(allowedMints);
  //       expect(allowedMintsData.mints).to.eql([mint1, mint2, newMint1, newMint2]);
  //
  //
  // });
  it("Buyers can add payment", async () => {

      const mintAuth = web3.PublicKey.unique();
      const gaugemeister = web3.PublicKey.unique()
      const config = web3.Keypair.generate();
      const [allowedMints, _] = web3.PublicKey.findProgramAddressSync([Buffer.from("allow-list"), config.publicKey.toBuffer()], program.programId);
      let balance = 0;
      console.log(mintAuth.toBase58())
      const mint = await createMint( program.provider.connection,
            payer,
            mintAuth,
            null,
            9,
          undefined,
          {
              commitment: "confirmed",
          }
        );

      const mintAccount = await program.provider.connection.getAccountInfo(mint);
      expect(mintAccount.data.length).to.eql(MintLayout.span);

      await program.methods.createConfig(
          [mint],
          gaugemeister,
          new BN(100),
          program.provider.publicKey,
      ).accounts(
          {
              config: config.publicKey,
              payer: program.provider.publicKey,
              allowedMints
          }).signers([config]).rpc();
        const allowedMintsData = await program.account.allowedMints.fetch(allowedMints);
        expect(allowedMintsData.mints[0]).to.eql(mint);
  });
  it("Sellers can withdraw vote payment", async () => {
  });
  it("Buyers can withdraw rewards", async () => {
  });
  it("Sellers can withdraw rewards", async () => {
  });
});

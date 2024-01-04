import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import {VoteMarket} from "../target/types/vote_market";
import {expect} from "chai";
import BN from "bn.js";
import {setupConfig} from "./test-setup";

describe("vote-market", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.VoteMarket as Program<VoteMarket>;

  it("Creates a config account", async () => {
    const {config, allowedMints, mint1, mint2, gaugemeister, scriptAuthority, tx} = await setupConfig(program);
    const configAccount = await program.account.voteMarketConfig.fetch(config.publicKey);
    expect(configAccount.gaugemeister).to.eql(gaugemeister);
    expect(configAccount.scriptAuthority).to.eql(scriptAuthority);
    expect(configAccount.efficiencyRatio.eq(new BN(100))).to.be.true;
    const allowedMintsAccount = await program.account.allowedMints.fetch(allowedMints);
    expect(allowedMintsAccount.mints).to.eql([mint1, mint2]);
    console.log(tx);
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

  });
  it("Buyers can add payment", async () => {
  });
  it("Sellers can withdraw vote payment", async () => {
  });
  it("Buyers can withdraw rewards", async () => {
  });
  it("Sellers can withdraw rewards", async () => {
  });
});

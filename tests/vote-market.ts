import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { VoteMarket } from "../target/types/vote_market";
import {expect} from "chai";
import BN from "bn.js";

describe("vote-market", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.VoteMarket as Program<VoteMarket>;

  it("Creates a config account", async () => {

    const config = web3.Keypair.generate();
    const[allowedMints, _] = web3.PublicKey.findProgramAddressSync([Buffer.from("allow-list"), config.publicKey.toBuffer()], program.programId);
    const mint1 = web3.PublicKey.unique();
    const mint2 = web3.PublicKey.unique();
    const gaugemeister = web3.PublicKey.unique()
    const scriptAuthority = web3.PublicKey.unique()
    console.log(program.provider.publicKey.toBase58())
    const tx = await program.methods.createConfig(
         [mint1, mint2],
          gaugemeister,
        new BN(100),
         scriptAuthority,
    ).accounts(
        {
          config: config.publicKey,
          payer: program.provider.publicKey,
          allowedMints
        }).signers([config]).rpc({skipPreflight: true});
    const configAccount = await program.account.voteMarketConfig.fetch(config.publicKey);
    expect(configAccount.gaugemeister).to.eql(gaugemeister);
    expect(configAccount.scriptAuthority).to.eql(scriptAuthority);
    expect(configAccount.efficiencyRatio.eq(new BN(100))).to.be.true;
    const allowedMintsAccount = await program.account.allowedMints.fetch(allowedMints);
    expect(allowedMintsAccount.mints).to.eql([mint1, mint2]);
    console.log(tx);
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

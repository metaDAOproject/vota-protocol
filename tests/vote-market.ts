import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VoteMarket } from "../target/types/vote_market";

describe("vote-market", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.VoteMarket as Program<VoteMarket>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
  it("Creates a config account", async () => {
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

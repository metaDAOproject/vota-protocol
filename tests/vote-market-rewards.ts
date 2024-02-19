import fs from "fs";
import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider, Program, web3 } from "@coral-xyz/anchor";
import { VoteMarket } from "../target/types/vote_market";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import GAUGE_IDL from "../external-state/idls/gauge.json";
import {
  GAUGE,
  GAUGE_PROGRAM_ID,
  GAUGEMEISTER,
  LOCKED_VOTER_PROGRAM_ID,
} from "./constants";
import { Gauge } from "../external-state/types/gauge";
import { setupTokens } from "./setup-tokens";
import { setupConfig } from "./test-setup";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccount,
  getAccount,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import BN from "bn.js";
import dotenv from "dotenv";
import { expect } from "chai";
import * as crypto from "crypto";
import { getVoteAccounts } from "./common";

dotenv.config();

describe("vote market rewards phase", () => {
  // Configure the client to use the local cluster.
  const rawKey = fs.readFileSync(process.env.KEY_PATH, "utf-8");
  const payer = web3.Keypair.fromSecretKey(Uint8Array.from(JSON.parse(rawKey)));

  const connection = new web3.Connection("http://127.0.0.1:8899", "confirmed");
  anchor.setProvider(
    new AnchorProvider(
      connection,
      new NodeWallet(payer),
      AnchorProvider.defaultOptions()
    )
  );
  const program = anchor.workspace.VoteMarket as Program<VoteMarket>;
  before(async () => {
    const latestBlockhash =
      await program.provider.connection.getLatestBlockhash();
    const sig = await program.provider.connection.requestAirdrop(
      payer.publicKey,
      1000000000000
    );
    await program.provider.connection.confirmTransaction({
      signature: sig,
      ...latestBlockhash,
    });
  });
  const gaugeProgram = new Program(
    GAUGE_IDL as any,
    GAUGE_PROGRAM_ID
  ) as Program<Gauge>;
  it("Vote sellers can withdraw vote payment", async () => {
    // Add payment
    const { mint, ata } = await setupTokens(program, payer);

    const config = web3.Keypair.fromSecretKey(
      Uint8Array.from(JSON.parse(fs.readFileSync("tests/config.json", "utf-8")))
    );
    const { allowedMints } = await setupConfig(program, [mint], config);

    var gaugeMeisterData = await gaugeProgram.account.gaugemeister.fetch(
      GAUGEMEISTER
    );
    const epochBuffer = Buffer.alloc(4);
    epochBuffer.writeUInt32LE(gaugeMeisterData.currentRewardsEpoch + 1);
    const [voteBuy, _] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("vote-buy"),
        epochBuffer,
        config.publicKey.toBuffer(),
        GAUGE.toBuffer(),
      ],
      program.programId
    );
    const tokenVault = getAssociatedTokenAddressSync(mint, voteBuy, true);

    await program.methods
      .increaseVoteBuy(
        gaugeMeisterData.currentRewardsEpoch + 1,
        new BN(1_000_000)
      )
      .accounts({
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
      })
      .rpc({ commitment: "confirmed" });
    let {
      voteDelegate,
      escrow,
      gaugeVoter,
      gaugeVote,
      epochGaugeVoter,
      epochGaugeVote,
      epochGauge,
    } = getVoteAccounts(
      config,
      program,
      gaugeMeisterData,
      program.provider.publicKey
    );

    const newAdmin = web3.Keypair.generate();
    const updateAdminSig = await program.methods
      .updateAdmin(newAdmin.publicKey)
      .accounts({
        config: config.publicKey,
        admin: program.provider.publicKey,
      })
      .rpc();

    const treasury = await createAssociatedTokenAccount(
      connection,
      payer,
      mint,
      newAdmin.publicKey
    );

    try {
      // Claim payment
      await program.methods
        .claimVotePayment(gaugeMeisterData.currentRewardsEpoch + 1)
        .accounts({
          scriptAuthority: program.provider.publicKey,
          seller: program.provider.publicKey,
          sellerTokenAccount: ata,
          tokenVault,
          treasury,
          admin: newAdmin.publicKey,
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
        })
        .rpc();
      expect.fail(
        "Should have thrown an error when withdrawing vote payment before voting is complete"
      );
    } catch (e) {
      expect(e.error.errorCode.code).to.equal("EpochVotingNotCompleted");
    }

    //get instruction discriminator
    var hash = crypto.createHash("sha256");
    hash.update(Buffer.from("global:trigger_next_epoch"));
    const discriminator = hash.digest().subarray(0, 8);
    const triggerNextEpochIx = new web3.TransactionInstruction({
      data: Buffer.from(discriminator),
      keys: [{ pubkey: GAUGEMEISTER, isSigner: false, isWritable: true }],
      programId: GAUGE_PROGRAM_ID,
    });
    const latestBlock = await program.provider.connection.getLatestBlockhash();
    const triggerNextEpochTx = new web3.Transaction({
      feePayer: payer.publicKey,
      ...latestBlock,
    }).add(triggerNextEpochIx);
    triggerNextEpochTx.sign(payer);
    await program.provider.sendAndConfirm(triggerNextEpochTx, [], {
      commitment: "confirmed",
    });
    let sellerTokenAccountData = await getAccount(
      program.provider.connection,
      ata
    );
    expect(sellerTokenAccountData.amount === BigInt(999_000_000)).to.be.true;
    let tokenVaultData = await getAccount(
      program.provider.connection,
      tokenVault
    );
    expect(tokenVaultData.amount === BigInt(1_000_000)).to.be.true;

    const nonSellerPayer = web3.Keypair.generate();
    await program.provider.connection.requestAirdrop(
      nonSellerPayer.publicKey,
      1000000000000
    );

    const updateScriptAuthSig = await program.methods
      .updateScriptAuthority(nonSellerPayer.publicKey)
      .accounts({
        config: config.publicKey,
        admin: newAdmin.publicKey,
      })
      .signers([newAdmin])
      .rpc();
    const latestBlockhash =
      await program.provider.connection.getLatestBlockhash();
    await connection.confirmTransaction(
      {
        signature: updateScriptAuthSig,
        ...latestBlockhash,
      },
      "confirmed"
    );

    // Can't claim until a max amount is set
    await program.methods
      .setMaxAmount(gaugeMeisterData.currentRewardsEpoch + 1, new BN(1_000_000))
      .accounts({
        config: config.publicKey,
        gauge: GAUGE,
        voteBuy,
        scriptAuthority: nonSellerPayer.publicKey,
      })
      .signers([nonSellerPayer])
      .rpc();

    await program.methods
      .claimVotePayment(gaugeMeisterData.currentRewardsEpoch + 1)
      .accounts({
        scriptAuthority: nonSellerPayer.publicKey,
        seller: program.provider.publicKey,
        sellerTokenAccount: ata,
        tokenVault,
        treasury,
        admin: newAdmin.publicKey,
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
      })
      .signers([nonSellerPayer])
      .rpc({ commitment: "confirmed" });
    sellerTokenAccountData = await getAccount(program.provider.connection, ata);
    const expectedRewards = BigInt(18514);
    const expectedFee = (expectedRewards * BigInt(600)) / BigInt(10_000);
    expect(sellerTokenAccountData.amount).to.equal(
      BigInt(999_000_000) + expectedRewards - expectedFee
    );
    tokenVaultData = await getAccount(program.provider.connection, tokenVault);
    expect(tokenVaultData.amount).to.equal(BigInt(1_000_000) - expectedRewards);

    const treasuryAccount = await getAccount(
      program.provider.connection,
      treasury
    );
    expect(treasuryAccount.amount).to.equal(expectedFee);
    const voteDelegateBalance = await program.provider.connection.getBalance(
      voteDelegate
    );
    const expectedGaugeVoteRent =
      await program.provider.connection.getMinimumBalanceForRentExemption(16);
    expect(voteDelegateBalance).to.equal(expectedGaugeVoteRent);

    //Should not be able to claim again
    try {
      await program.methods
        .claimVotePayment(gaugeMeisterData.currentRewardsEpoch + 1)
        .accounts({
          scriptAuthority: nonSellerPayer.publicKey,
          seller: program.provider.publicKey,
          sellerTokenAccount: ata,
          tokenVault,
          treasury,
          admin: newAdmin.publicKey,
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
        })
        .signers([nonSellerPayer])
        .rpc({ commitment: "confirmed" });
      expect.fail("Claimed vote payment twice");
    } catch (e) {
      expect(e.error.errorCode.code).to.equal("AccountNotInitialized");
    }
  });
  it("Buyers can get refund of excess vote buy tokens", async () => {
    const { mint, ata } = await setupTokens(program, payer);

    const { allowedMints, config } = await setupConfig(program, [mint]);

    var gaugeMeisterData = await gaugeProgram.account.gaugemeister.fetch(
      GAUGEMEISTER
    );
    const epochBuffer = Buffer.alloc(4);
    epochBuffer.writeUInt32LE(gaugeMeisterData.currentRewardsEpoch + 1);
    const [voteBuy, _] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("vote-buy"),
        epochBuffer,
        config.publicKey.toBuffer(),
        GAUGE.toBuffer(),
      ],
      program.programId
    );
    const tokenVault = getAssociatedTokenAddressSync(mint, voteBuy, true);

    await program.methods
      .increaseVoteBuy(
        gaugeMeisterData.currentRewardsEpoch + 1,
        new BN(1_000_000)
      )
      .accounts({
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
      })
      .rpc({ commitment: "confirmed" });
    let buyerTokenAccountData = await getAccount(
      program.provider.connection,
      ata
    );
    expect(buyerTokenAccountData.amount).to.equal(BigInt(999_000_000));
    await program.methods
      .setMaxAmount(gaugeMeisterData.currentRewardsEpoch + 1, new BN(700_000))
      .accounts({
        config: config.publicKey,
        gauge: GAUGE,
        voteBuy,
        scriptAuthority: program.provider.publicKey,
      })
      .rpc({ commitment: "confirmed" });

    const sig = await program.methods
      .voteBuyRefund(gaugeMeisterData.currentRewardsEpoch + 1)
      .accounts({
        buyer: program.provider.publicKey,
        buyerTokenAccount: ata,
        tokenVault,
        voteBuy,
        mint,
        config: config.publicKey,
        gauge: GAUGE,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    let blockHash = await program.provider.connection.getLatestBlockhash();
    await program.provider.connection.confirmTransaction(
      {
        signature: sig,
        ...blockHash,
      },
      "confirmed"
    );
    buyerTokenAccountData = await getAccount(program.provider.connection, ata);
    expect(buyerTokenAccountData.amount).to.equal(
      BigInt(999_000_000 + 300_000)
    );
  });
});

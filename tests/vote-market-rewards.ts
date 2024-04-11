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
import { getVoteAccounts, triggerNextEpoch } from "./common";

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
    await triggerNextEpoch(program, payer);
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
        .signers([nonSellerPayer, payer])
        .rpc();
    sellerTokenAccountData = await getAccount(program.provider.connection, ata);
    const expectedRewards = BigInt(18204);
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

    try {
        const wrongBuyer = web3.Keypair.generate();
        await connection.requestAirdrop(wrongBuyer.publicKey, 1000000000);
        await new Promise((resolve) => setTimeout(resolve, 1500));
        const wrongAta = await createAssociatedTokenAccount(
            connection,
            wrongBuyer,
            mint,
            wrongBuyer.publicKey
        );
        const sig = await program.methods
            .voteBuyRefund(gaugeMeisterData.currentRewardsEpoch + 1)
            .signers([wrongBuyer])
            .accounts({
                buyer: wrongBuyer.publicKey,
                buyerTokenAccount: wrongAta,
                tokenVault,
                voteBuy,
                mint,
                config: config.publicKey,
                gauge: GAUGE,
                gaugemeister: GAUGEMEISTER,
                tokenProgram: TOKEN_PROGRAM_ID,
            })
            .rpc({
                skipPreflight: true,
            });
        let blockHash = await program.provider.connection.getLatestBlockhash();
        await program.provider.connection.confirmTransaction(
            {
                signature: sig,
                ...blockHash,
            },
            "confirmed"
        );
        expect.fail("Wrong account successfully took the refund");
    } catch (e) {
        expect(e.error.errorCode.code).to.equal("ConstraintHasOne");
    }
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
        gaugemeister: GAUGEMEISTER,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc({
        skipPreflight: true,
      });
    let blockHash = await program.provider.connection.getLatestBlockhash();
    await program.provider.connection.confirmTransaction(
      {
        signature: sig,
        ...blockHash,
      },
      "confirmed"
    );
    buyerTokenAccountData = await getAccount(program.provider.connection, ata);
    // Vote buys in excess of the max are refunded
    expect(buyerTokenAccountData.amount).to.equal(
      BigInt(999_000_000 + 300_000)
    );
    // make sure the buyer can't get double refunds
    try {
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
          gaugemeister: GAUGEMEISTER,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc({ skipPreflight: true });
      let blockHash = await program.provider.connection.getLatestBlockhash();
      await program.provider.connection.confirmTransaction(
        {
          signature: sig,
          ...blockHash,
        },
        "confirmed"
      );
    } catch (e) {
      expect(e.error.errorCode.code).to.equal("InvalidRefund");
    }
    //epochs are set to 1 second duration for the test
    //Get the rest refunded after the rewards epoch ends
    const gaugemeisterData = await gaugeProgram.account.gaugemeister.fetch(
      GAUGEMEISTER
    );
    await new Promise((resolve) => setTimeout(resolve, 1500));
    // Votes are on rewards epoch
    await triggerNextEpoch(program, payer);
    await new Promise((resolve) => setTimeout(resolve, 1500));
    // Rewards epoch complete
    await triggerNextEpoch(program, payer);
    await new Promise((resolve) => setTimeout(resolve, 1500));
    const sig2 = await program.methods
      .voteBuyRefund(gaugeMeisterData.currentRewardsEpoch + 1)
      .accounts({
        buyer: program.provider.publicKey,
        buyerTokenAccount: ata,
        tokenVault,
        voteBuy,
        mint,
        config: config.publicKey,
        gauge: GAUGE,
        gaugemeister: GAUGEMEISTER,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc({ skipPreflight: false });
    let blockHash2 = await program.provider.connection.getLatestBlockhash();
    await program.provider.connection.confirmTransaction(
      {
        signature: sig2,
        ...blockHash2,
      },
      "confirmed"
    );
    let buyerTokenAccountData2 = await getAccount(
      program.provider.connection,
      ata
    );
    // All vote buys are refunded
    expect(buyerTokenAccountData2.amount).to.equal(
      BigInt(999_000_000 + 1_000_000)
    );
  });
});

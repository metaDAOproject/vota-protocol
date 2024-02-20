import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import {
  GAUGE,
  GAUGE_PROGRAM_ID,
  GAUGEMEISTER,
  LOCKED_VOTER_PROGRAM_ID,
} from "./constants";
import { VoteMarket } from "../target/types/vote_market";
import crypto from "crypto";

export function getVoteAccounts(
  config: web3.Keypair,
  program: Program<VoteMarket>,
  gaugeMeisterData,
  owner: web3.PublicKey
) {
  const epochBuffer = Buffer.alloc(4);
  epochBuffer.writeUInt32LE(gaugeMeisterData.currentRewardsEpoch + 1);

  let [voteDelegate, voteDelegateBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vote-delegate"), config.publicKey.toBuffer()],
      program.programId
    );

  let [escrow, escrowBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("Escrow"),
      gaugeMeisterData.locker.toBuffer(),
      owner.toBuffer(),
    ],
    LOCKED_VOTER_PROGRAM_ID
  );

  let [gaugeVoter, gaugeVoterBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("GaugeVoter"), GAUGEMEISTER.toBuffer(), escrow.toBuffer()],
      GAUGE_PROGRAM_ID
    );

  let [gaugeVote, gaugeVoteBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("GaugeVote"), gaugeVoter.toBuffer(), GAUGE.toBuffer()],
    GAUGE_PROGRAM_ID
  );

  let [epochGaugeVoter, epochGaugeVoterBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("EpochGaugeVoter"),
        gaugeVoter.toBuffer(),
        Buffer.from(epochBuffer),
      ],
      GAUGE_PROGRAM_ID
    );

  let [epochGaugeVote, epochGaugeVoteBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("EpochGaugeVote"),
        gaugeVote.toBuffer(),
        Buffer.from(epochBuffer),
      ],
      GAUGE_PROGRAM_ID
    );

  let [epochGauge, epochGaugeBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("EpochGauge"), GAUGE.toBuffer(), Buffer.from(epochBuffer)],
      GAUGE_PROGRAM_ID
    );
  return {
    voteDelegate,
    escrow,
    gaugeVoter,
    gaugeVote,
    epochGaugeVoter,
    epochGaugeVote,
    epochGauge,
  };
}

export async function triggerNextEpoch(
  program: Program<VoteMarket>,
  payer: web3.Keypair
) {
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
}

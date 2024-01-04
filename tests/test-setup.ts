import {Program, web3} from "@coral-xyz/anchor";
import BN from "bn.js";
import { VoteMarket } from "../target/types/vote_market";

export async function setupConfig(program: Program<VoteMarket>) {
    const config = web3.Keypair.generate();
    const [allowedMints, _] = web3.PublicKey.findProgramAddressSync([Buffer.from("allow-list"), config.publicKey.toBuffer()], program.programId);
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
        }).signers([config]).rpc();
    return {config, allowedMints, mint1, mint2, gaugemeister, scriptAuthority, tx};
}
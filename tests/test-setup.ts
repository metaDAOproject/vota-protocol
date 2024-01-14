import {Program, web3} from "@coral-xyz/anchor";
import BN from "bn.js";
import { VoteMarket } from "../target/types/vote_market";
import {GAUGEMEISTER} from "./constants";
import fs from "fs";

export async function setupConfig(program: Program<VoteMarket>, allowedMintList: web3.PublicKey[] = undefined, config: web3.Keypair = undefined)
{
    if(config === undefined){
        config = web3.Keypair.generate();
    }
    const [allowedMints, _] = web3.PublicKey.findProgramAddressSync([Buffer.from("allow-list"), config.publicKey.toBuffer()], program.programId);
    if(allowedMintList === undefined) {
        const mint1 = web3.PublicKey.unique();
        const mint2 = web3.PublicKey.unique();
        allowedMintList = [mint1, mint2];
    }
    const gaugemeister = GAUGEMEISTER;
    const scriptAuthority = program.provider.publicKey;
    const configAccount = await program.provider.connection.getAccountInfo(config.publicKey);
    if(!configAccount) {
        const tx = await program.methods.createConfig(
            allowedMintList,
            gaugemeister,
            new BN(100),
            scriptAuthority,
        ).accounts(
            {
                config: config.publicKey,
                payer: program.provider.publicKey,
                allowedMints
            }).signers([config]).rpc();
        await new Promise(resolve => setTimeout(resolve, 1000));
    }
    return {config, allowedMints, allowedMintList, scriptAuthority};
}
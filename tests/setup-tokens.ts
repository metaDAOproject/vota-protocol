import {Program, web3} from "@coral-xyz/anchor";
import {createAssociatedTokenAccount, createMint, getAccount, MintLayout, mintTo} from "@solana/spl-token";
import {expect} from "chai";
import { VoteMarket } from "../target/types/vote_market";

export async function setupTokens(program: Program<VoteMarket>, payer: web3.Keypair) {
    const mintAuth = web3.Keypair.generate();
    const mint = await createMint(program.provider.connection,
        payer,
        mintAuth.publicKey,
        null,
        9,
        undefined,
        {
            commitment: "confirmed",
        }
    );

    const mintAccount = await program.provider.connection.getAccountInfo(mint);
    expect(mintAccount.data.length).to.eql(MintLayout.span);
    const ata = await createAssociatedTokenAccount(
        program.provider.connection,
        payer,
        mint,
        program.provider.publicKey
    );
    const sig = await mintTo(
        program.provider.connection,
        payer,
        mint,
        ata,
        mintAuth,
        BigInt(1000000000),
        [],
        {
            skipPreflight: true,
        }
    );
    console.log("minto to sig", sig)
    const tokenAccount = await getAccount(program.provider.connection, ata);
    console.log("this is the amount", tokenAccount.amount);
    expect(tokenAccount.amount).to.eql(BigInt(1000000000));
    return {mint, ata};
}
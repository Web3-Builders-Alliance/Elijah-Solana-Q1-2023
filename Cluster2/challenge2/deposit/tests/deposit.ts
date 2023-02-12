import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Deposit } from "../target/types/deposit";
import * as web3 from "@solana/web3.js"

describe("deposit program", () => {
    const anchorProvider = anchor.AnchorProvider.env();
    anchor.setProvider(anchorProvider);
    const program = anchor.workspace.Deposit as Program<Deposit>;

    const initializer_pubkey = anchorProvider.wallet.publicKey;
    const [vault_pda, _] = web3.PublicKey.findProgramAddressSync([Buffer.from("vault", "binary"), initializer_pubkey.toBuffer()], program.programId);

    const LAMPORTS_PER_SOL = anchor.web3.LAMPORTS_PER_SOL;
    const amount_to_deposit: anchor.BN = new anchor.BN(0.2 * anchor.web3.LAMPORTS_PER_SOL);
    const amount_to_withdraw: anchor.BN = new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL);

    it("Inits a vault", async () => {
        const tx = await program.methods.initialize()
            .accounts({
                initializer: anchorProvider.wallet.publicKey,
                vault: vault_pda,
                systemProgram: web3.SystemProgram.programId
            })
            .rpc();
        console.log("Transaction signature", tx);
    });

    it("Deposits to vault", async () => {
        const tx_send = await program.methods.deposit(amount_to_deposit)
            .accounts({
                initializer: initializer_pubkey,
                vault: vault_pda,
                systemProgram: web3.SystemProgram.programId
            })
            .rpc();
        console.log(`Deposited ${amount_to_deposit.toNumber() / LAMPORTS_PER_SOL} SOL from ${initializer_pubkey} to the vault: ${vault_pda}.`);
        console.log("Transaction signature", tx_send);
    });

    it("Withdraws from vault", async () => {
        const tx_send = await program.methods.withdraw(amount_to_withdraw)
            .accounts({
                initializer: initializer_pubkey,
                vault: vault_pda
            })
            .rpc();
        console.log(`Withdrawal ${amount_to_withdraw.toNumber() / LAMPORTS_PER_SOL} SOL from vault to ${initializer_pubkey}.`);
        console.log("Transaction signature", tx_send);
      });
});
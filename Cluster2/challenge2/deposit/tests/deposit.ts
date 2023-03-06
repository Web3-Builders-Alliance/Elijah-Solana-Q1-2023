import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Deposit } from "../target/types/deposit";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync, getMint, getOrCreateAssociatedTokenAccount, mintToChecked, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { execSync } from "child_process";
import { keypairIdentity, Metaplex, toBigNumber } from "@metaplex-foundation/js";

describe("deposit", () => {
    let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
    const program = anchor.workspace.Deposit as Program<Deposit>;
    const deposit_account = anchor.web3.Keypair.generate();
    const deposit_auth = anchor.web3.Keypair.generate();
    const metaplex = Metaplex.make(provider.connection).use(keypairIdentity(deposit_auth));
    let mint = anchor.web3.Keypair.generate();
    let usdc_auth = anchor.web3.Keypair.generate();

    const TOKEN_METADATA_PROGRAM_ID = anchor.web3.Keypair.generate();

    let [pda_auth, pda_bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            anchor.utils.bytes.utf8.encode("auth"),
            deposit_account.publicKey.toBuffer()
        ],
        program.programId
    );

    let [sol_vault, sol_bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            anchor.utils.bytes.utf8.encode("sol_vault"),
            pda_auth.toBuffer()
        ],
        program.programId
    );

    // execSync(
    //     `anchor idl init --filepath target/idl/deposit.json ${program.programId}`,
    //     { stdio: "inherit" }
    // );

    before(async () => {
        let latestBlockHash = await provider.connection.getLatestBlockhash();

        let res = await provider.connection.requestAirdrop(
            deposit_auth.publicKey,
            100 * anchor.web3.LAMPORTS_PER_SOL
        );

        await provider.connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: res,
        });

        console.log("Deposit_auth balance: ", await provider.connection.getBalance(deposit_auth.publicKey));
        
    });

    it("Is initialized!", async () => {
        const tx = await program.methods.initialize()
            .accounts({
                depositAccount: deposit_account.publicKey,
                pdaAuth: pda_auth,
                depositAuth: deposit_auth.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            }).signers([deposit_account, deposit_auth]).rpc();

        console.log("Your transaction signature", tx);

        let result = await program.account.vault.fetch(deposit_account.publicKey);
        console.log(result);
    });

    it("Deposits native SOL", async () => {
        const deposit_amount = new anchor.BN(25 * anchor.web3.LAMPORTS_PER_SOL);
        const deposit_native_tx = await program.methods.deposit(deposit_amount)
            .accounts({
                depositAccount: deposit_account.publicKey,
                pdaAuth: pda_auth,
                solVault: sol_vault,
                depositAuth: deposit_auth.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            }).signers([deposit_auth]).rpc();

        let sol_vault_lamps = await provider.connection.getBalance(sol_vault);
        console.log(sol_vault_lamps);

        let result = await program.account.vault.fetch(deposit_account.publicKey);
        console.log(result);
    });

    it("Withdraws native SOL", async () => {
        let withdraw_amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);

        const withdraw_native_tx = await program.methods.withdraw(withdraw_amount)
            .accounts({
                depositAccount: deposit_account.publicKey,
                pdaAuth: pda_auth,
                solVault: sol_vault,
                depositAuth: deposit_auth.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            }).signers([deposit_auth]).rpc();

        let sol_vault_lamps = await provider.connection.getBalance(sol_vault);
        console.log(sol_vault_lamps);
    });

    it("Create mock SPL Token", async () => {

        let token_mint = await createMint(
            provider.connection,
            deposit_auth,
            usdc_auth.publicKey,
            usdc_auth.publicKey,
            6,
            mint,
            null,
            TOKEN_PROGRAM_ID
        );

        let test = await getMint(provider.connection, token_mint, null, TOKEN_PROGRAM_ID);
        console.log(test);

        let deposit_auth_usdc_acct = await getOrCreateAssociatedTokenAccount(provider.connection, deposit_auth, token_mint, deposit_auth.publicKey, false, undefined, undefined, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID)

        let mint_to_sig = await mintToChecked(provider.connection, deposit_auth, token_mint, deposit_auth_usdc_acct.address, usdc_auth, 200e6, 6, [], undefined, TOKEN_PROGRAM_ID);

        console.log(mint_to_sig);

    });

    it("Deposits SPL Token", async () => {
        let to_token_acct = getAssociatedTokenAddressSync(mint.publicKey, pda_auth, true, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);

        let from_token_acct = getOrCreateAssociatedTokenAccount(provider.connection, deposit_account, mint.publicKey, deposit_auth.publicKey, false);
        let from_token_acct_wallet = (await from_token_acct).address;


        let deposit_spl_tx = await program.methods.depositSpl(new anchor.BN(25e6)).accounts({
            depositAccount: deposit_account.publicKey,
            pdaAuth: pda_auth,
            depositAuth: deposit_auth.publicKey,
            toTokenAcct: to_token_acct,
            fromTokenAcct: from_token_acct_wallet,
            tokenMint: mint.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).signers([deposit_auth]).rpc();

        console.log(deposit_spl_tx);

    });

    // not sure about the implementation, hence I don't run it for now
    xit("Withdraws SPL Token", async () => {
        let to_token_acct = getAssociatedTokenAddressSync(mint.publicKey, pda_auth, true, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);

        let from_token_acct = getOrCreateAssociatedTokenAccount(provider.connection, deposit_account, mint.publicKey, deposit_auth.publicKey, false);
        let from_token_acct_wallet = (await from_token_acct).address;

        let withdraw_spl_tx = await program.methods.withdrawSpl(new anchor.BN(25e6)).accounts({
            depositAccount: deposit_account.publicKey,
            pdaAuth: pda_auth,
            depositAuth: deposit_auth.publicKey,
            toTokenAcct: to_token_acct,
            fromTokenAcct: from_token_acct_wallet,
            tokenMint: mint.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).signers([deposit_auth]).rpc();

        console.log(withdraw_spl_tx);
    });

    it("mints 1 NFT & sends it", async () => {
        //creates a metadata
        const { uri } = await metaplex.nfts().uploadMetadata({
            name: "WBA Diploma #441",
            description: "The NFT represents the skillset of the WBA graduate",
            image: "https://arweave.net/123",
            github: "https://github.com/ilyxabatko",
            skillset: "Rust, Native Solana, Anchor, TypeScript, Mocha.js, React.js, Next.js"
        });

        // mints 1 nft
        let nft_mint = await metaplex.nfts().create({
            name: "WBA Diploma #441",
            symbol: "WBAD",
            uri: uri,
            sellerFeeBasisPoints: 300, // 3%
            isMutable: false,
            maxSupply: toBigNumber(1)
        });

        // gets and prints mint's info
        const nft_mint_info = await getMint(provider.connection, nft_mint.mintAddress, null, TOKEN_PROGRAM_ID);
        console.log("NFT MINT INFO: ", nft_mint_info);

        // Derives the metadata and master edition addresses 

        const metadata_address = (anchor.web3.PublicKey.findProgramAddressSync([
                Buffer.from("metadata"),
                TOKEN_METADATA_PROGRAM_ID.publicKey.toBuffer(),
                nft_mint.mintAddress.toBuffer()
            ],
            TOKEN_METADATA_PROGRAM_ID.publicKey
        ))[0];
        console.log("Metadata initialized: ", metadata_address);

        const master_edition_address = (anchor.web3.PublicKey.findProgramAddressSync([
                Buffer.from("metadata"),
                TOKEN_METADATA_PROGRAM_ID.publicKey.toBuffer(),
                nft_mint.mintAddress.toBuffer(),
                Buffer.from("edition")
            ],
            TOKEN_METADATA_PROGRAM_ID.publicKey
        ))[0];
        console.log("Master edition initialized: ", master_edition_address);

        let to_token_acct = getAssociatedTokenAddressSync(nft_mint.mintAddress, pda_auth, true, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);

        // sends an NFT to an ATA 
        let deposit_nft_tx = await program.methods.depositSpl(new anchor.BN(1)).accounts({
            depositAccount: deposit_account.publicKey,
            pdaAuth: pda_auth,
            depositAuth: deposit_auth.publicKey,
            toTokenAcct: to_token_acct,
            fromTokenAcct: deposit_account.publicKey,
            tokenMint: mint.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).signers([deposit_auth]).rpc();

        console.log(deposit_nft_tx);

    });

});
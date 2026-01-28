import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VoteApp } from "../target/types/vote_app";
import { program } from "@coral-xyz/anchor/dist/cjs/native/system";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { expect } from "chai";
import { getAccount, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";


const SEEDS = {
 TREASURY_CONFIG: "treasury_config",
 X_MINT: "x_mint",
 SOL_VAULT: "sol_vault",
 MINT_AUTHORITY: "mint_authority",
 VOTER: "voter",
 PROPOSAL_COUNTER: "proposal_counter",
 PROPOSAL: "proposal"


} as const

const PROPOSAL_ID = 1
const findPda = (programId: anchor.web3.PublicKey, seeds: (Buffer | Uint8Array)[]): anchor.web3.PublicKey => {
 const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, programId);
 return pda


}
const getBlockTime = async (connection: anchor.web3.Connection): Promise<number> => {
 const slot = await connection.getSlot();
 const blockTime = await connection.getBlockTime(slot)
 if (blockTime == null) {
  throw new Error("failed to fetch bock time ")
 }
 return blockTime
}
const airDropSol = async (connection: anchor.web3.Connection, publicKey: anchor.web3.PublicKey, sol: number) => {
 const signature = await connection.requestAirdrop(publicKey, sol);
 await connection.confirmTransaction(signature, "confirmed");
}
describe("1. initialized", () => {
 // Configure the client to use the local cluster.
 const provider = anchor.AnchorProvider.env()
 const connection = provider.connection;
 anchor.setProvider(provider)


 const program = anchor.workspace.voteApp as Program<VoteApp>;


 const adminWallet = (provider.wallet as NodeWallet).payer // fee payer 
 let proposalCreatorWallet = new anchor.web3.Keypair();
 let voterWallet = new anchor.web3.Keypair();
 let treausryConfigPda: anchor.web3.PublicKey
 let proposalCreatorTokenAccount: anchor.web3.PublicKey;
 let xMintPda: anchor.web3.PublicKey
 let solVaultPda: anchor.web3.PublicKey
 let voterPda: anchor.web3.PublicKey
 let mintAuthorityPda: anchor.web3.PublicKey


 let proposalAccountPda: anchor.web3.PublicKey
 let proposalCounterPda: anchor.web3.PublicKey
 let treausryTokenAccount: anchor.web3.PublicKey
 let voterTokenAccount: anchor.web3.PublicKey;

 beforeEach(async () => {
  // runs every test filed
  treausryConfigPda = findPda(program.programId, [anchor.utils.bytes.utf8.encode(SEEDS.TREASURY_CONFIG)])
  xMintPda = findPda(program.programId, [anchor.utils.bytes.utf8.encode(SEEDS.X_MINT)])
  solVaultPda = findPda(program.programId, [anchor.utils.bytes.utf8.encode(SEEDS.SOL_VAULT)])
  mintAuthorityPda = findPda(program.programId, [anchor.utils.bytes.utf8.encode(SEEDS.MINT_AUTHORITY)])
  proposalAccountPda = findPda(program.programId, [anchor.utils.bytes.utf8.encode(SEEDS.PROPOSAL), Buffer.from([PROPOSAL_ID])])
  voterPda = findPda(program.programId, [anchor.utils.bytes.utf8.encode(SEEDS.VOTER), voterWallet.publicKey.toBuffer()])
  proposalCounterPda = findPda(program.programId, [anchor.utils.bytes.utf8.encode(SEEDS.PROPOSAL_COUNTER)])
  console.log("Transferring sol tokens.....");
  await Promise.all([
   airDropSol(connection, proposalCreatorWallet.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL),
   airDropSol(connection, voterWallet.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL),
  ])
  console.log("Transfer of SOL successful");
 })
 const createTokenAccounts = async () => {
  treausryTokenAccount = (await getOrCreateAssociatedTokenAccount(
   connection,
   adminWallet,
   xMintPda,
   adminWallet.publicKey
  )).address
  proposalCreatorTokenAccount = (await getOrCreateAssociatedTokenAccount(
   connection,
   proposalCreatorWallet,
   xMintPda,
   proposalCreatorWallet.publicKey
  )).address;
  voterTokenAccount = (await getOrCreateAssociatedTokenAccount(
   connection,
   voterWallet,
   xMintPda,
   voterWallet.publicKey
  )).address;

 }

 it("It's initialized! treausry", async () => {
  // Add your test here.
  const solPrice = new anchor.BN(1000_000_000);
  const tokenPerPurchase = new anchor.BN(1000_000_000)

  console.log("Treasury Config PDA", treausryConfigPda)
  await program.methods.initializeTreasury(
   solPrice, tokenPerPurchase
  ).accounts({
   authority: adminWallet.publicKey,

  }).rpc();
  const treausryAccountData = await program.account.treasuryConfig.fetch(treausryConfigPda);
  expect(treausryAccountData.solPrice.toNumber()).equal(solPrice.toNumber());
  expect(treausryAccountData.tokenPerPurchase.toNumber()).to.equal(tokenPerPurchase.toNumber());
  expect(treausryAccountData.authority.toBase58()).to.equal(adminWallet.publicKey.toBase58())
  expect(treausryAccountData.xMint.toBase58()).to.equal(xMintPda.toBase58())
  await createTokenAccounts()
 });

 describe("2. Buy tokens ", () => {

  it("buys tokens!", async () => {
   // Add your test here.
   const tokenBalanceBefore = (await getAccount(connection, proposalCreatorTokenAccount)).amount
   await program.methods.buyTokens(

   ).accounts({
    buyer: proposalCreatorWallet.publicKey,
    treasuryTokenAccount: treausryTokenAccount,
    buyerTokenAccount: proposalCreatorTokenAccount,
    xMint: xMintPda
   }).signers([proposalCreatorWallet]).rpc();
   const tokenBalanceAfter = (await getAccount(connection, proposalCreatorTokenAccount)).amount;
   expect(tokenBalanceAfter - tokenBalanceBefore).to.equal(BigInt(1000_000_000));
  });
    it("2.2 buys tokens for voter!", async () => {
      const tokenBalanceBefore = (await getAccount(connection,voterTokenAccount)).amount;

      await program.methods.buyTokens().accounts({
       buyer:voterWallet.publicKey,
       treasuryTokenAccount:treausryTokenAccount,
       buyerTokenAccount:voterTokenAccount,
       xMint:xMintPda,
      }).signers([voterWallet]).rpc();

      const tokenBalanceAfter = (await getAccount(connection,voterTokenAccount)).amount;
      expect(tokenBalanceAfter-tokenBalanceBefore).to.equal(BigInt(1000_000_000));
    });

 })


 describe("3. voter ", () => {

  it("3.1 it register voters!", async () => {
   // Add your test here.
   await program.methods.registerVoter(
   ).accounts({
    authority: voterWallet.publicKey
   }).signers([voterWallet]).rpc();
   const voterAccountData = await program.account.voter.fetch(voterPda);
   expect(voterAccountData.voterId.toBase58()).to.equal(voterWallet.publicKey.toBase58())
  });

 })
 describe("3. voter ", () => {

  it("3.1 it register voters!", async () => {
   // Add your test here.
   const currentBlockTime = await getBlockTime(connection)
   const deadlineTime = new anchor.BN(currentBlockTime + 10);
   const proposalInfo = "Build a layer 2 solution";
   const stakeAmount = new anchor.BN(1000);
   await program.methods.registerProposal(
    proposalInfo, deadlineTime, stakeAmount
   ).accounts({
    authority: proposalCreatorWallet.publicKey,
    proposaleTokenAccount: proposalCreatorTokenAccount,
    proposalCounterAccount: proposalCounterPda,
    treasuryTokenAccount: treausryTokenAccount,
    xMint: xMintPda
   }).signers([proposalCreatorWallet]).rpc();
   const proposalAccountData = await program.account.proposal.fetch(proposalAccountPda);
   const proposalCounterAccountData = await program.account.proposalCounter.fetch(proposalCounterPda);
   expect(proposalCounterAccountData.proposalCount).to.equal(1)
   expect(proposalAccountData.authority.toBase58()).to.equal(proposalCreatorWallet.publicKey.toBase58());
   expect(proposalAccountData.deadline.toString()).to.equal(deadlineTime.toString());
   expect(proposalAccountData.numberOfVotes.toString()).to.equal("0");
   expect(proposalAccountData.proposalId.toString()).to.equal("1");
   expect(proposalAccountData.proposalInfo.toString()).to.equal("Build a layer 2 solution");
  });

 })
 describe("5.Casting Vote", () => {
  it("5.1 casts vote!", async () => {

   const stakeAmount = new anchor.BN(1000);
   await program.methods.proposalToVote(PROPOSAL_ID, stakeAmount).accounts({
    authority: voterWallet.publicKey,
    voterTokenAccount: voterTokenAccount,
    treasuryTokenAccount: treausryTokenAccount,
    xMint: xMintPda
   }).signers([voterWallet]).rpc();

  });
 })

});



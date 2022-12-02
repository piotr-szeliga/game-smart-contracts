import { createCloseAccountInstruction, NATIVE_MINT, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { Request, Response } from 'express';
import { getAta, getCreateAtaInstruction, getGameAddress, game_name, game_owner } from './utils';
import {
  Connection,
  clusterApiUrl,
  Keypair,
  PublicKey,
  Transaction,
  LAMPORTS_PER_SOL,
} from"@solana/web3.js";

import { AnchorProvider, Wallet, Program, setProvider, BN } from "@project-serum/anchor";
import { bs58 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { Plinko, IDL } from '../idl/plinko';

const plinko_idl = require("../idl/plinko.json");
const programId = plinko_idl.metadata.address;
const connection = new Connection(clusterApiUrl("devnet"));
const backendKp = Keypair.fromSecretKey(
  bs58.decode(process.env.BACKEND_SECRET_KEY || '')
);
const provider = new AnchorProvider(
  connection,
  new Wallet(backendKp),
  AnchorProvider.defaultOptions()
);
setProvider(provider);
const program = new Program(IDL, programId, provider) as Program<Plinko>;

export const getClaimTransaction = async (req: Request, res: Response) => {
  const { clientKey } = req.params;
  const claimer = new PublicKey(clientKey);
  
  // get claim amount from DB
  const claimAmount = 0.1; 

  const [game] = await getGameAddress(program.programId, game_name, game_owner);
  const gameData = await program.account.game.fetchNullable(game);
  if (!gameData) return res.status(500).json({ error: "Cannot get game data" });
  const mint = gameData.tokenMint;

  const transaction = new Transaction();

  const claimerAta = await getAta(mint, provider.wallet.publicKey);
  const instruction = await getCreateAtaInstruction(provider, claimerAta, mint, provider.wallet.publicKey);
  if (instruction) transaction.add(instruction);
  const gameTreasuryAta = await getAta(mint, game, true);
  transaction.add(
    program.transaction.claim(new BN(claimAmount * LAMPORTS_PER_SOL), {
      accounts: {
        claimer,
        backend: backendKp.publicKey,
        claimerAta,
        game,
        gameTreasuryAta,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    })
  );
  if (mint.toString() === NATIVE_MINT.toString()) {
    transaction.add(createCloseAccountInstruction(claimerAta, claimer, claimer));
  }
  
  transaction.feePayer = claimer;
  transaction.recentBlockhash = (await program.provider.connection.getLatestBlockhash("confirmed")).blockhash;

  console.log("Transaction made by BE:", transaction);
  transaction.partialSign(backendKp);
  console.log("Partial Signed Transaction by BE Keypair:", transaction);
  let serializedBuffer = transaction.serialize({ requireAllSignatures: false }).toString("base64");

  return res.json(serializedBuffer);
};

export const sendCalimTransaction = async (req: Request, res: Response) => {
  const { clientKey } = req.params;
  const { serializedBuffer } = req.body;
  const recoveredTx = Transaction.from(Buffer.from(serializedBuffer, "base64"));
  const txSignature = await program.provider.connection.sendRawTransaction(recoveredTx.serialize());
  await program.provider.connection.confirmTransaction(txSignature, "confirmed");
  console.log(txSignature);
  // Get claim amount from Transacction
  const programInstructions = recoveredTx.instructions.filter(instruction => instruction.programId.toString() === programId);
  const claimInstruction = programInstructions[0];
  const amountBytes = claimInstruction.data.slice(8).reverse();
  const amount = new BN(amountBytes);
  console.log("Claim amount: ", amount.toString());
  // Decrease clientKey's amount in DB

  return res.json({ txSignature, claimAmount: amount })
}

export const sendDepositTransaction = async (req: Request, res: Response) => {
  const { clientKey } = req.params;
  const { serializedBuffer } = req.body;
  const recoveredTx = Transaction.from(Buffer.from(serializedBuffer, "base64"));
  const txSignature = await program.provider.connection.sendRawTransaction(recoveredTx.serialize());
  await program.provider.connection.confirmTransaction(txSignature, "confirmed");
  console.log(txSignature);
  // Get deposit amount from Transacction
  const programInstructions = recoveredTx.instructions.filter(instruction => instruction.programId.toString() === programId);
  const claimInstruction = programInstructions[0];
  const amountBytes = claimInstruction.data.slice(8).reverse();
  const amount = new BN(amountBytes);
  console.log("Deposit amount: ", amount.toString());
  // Increase clientKey's amount in DB

  return res.json({ txSignature, depositAmount: amount })
}
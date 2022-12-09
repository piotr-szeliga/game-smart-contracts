import { createCloseAccountInstruction, NATIVE_MINT, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { Request, Response } from 'express';
import { getAta, getCreateAtaInstruction, getGameAddress, game_name, game_owner, backendKp, sendRequest } from './utils';
import { getPayload } from '../middleware/auth.middleware';
import {
  Connection,
  clusterApiUrl,
  Keypair,
  PublicKey,
  Transaction,
  LAMPORTS_PER_SOL,
} from"@solana/web3.js";

import { AnchorProvider, Wallet, Program, setProvider, BN } from "@project-serum/anchor";

const Plinko = require("../idl/plinko.json");
const programId = Plinko.metadata.address;
const connection = new Connection(clusterApiUrl("devnet"));
const provider = new AnchorProvider(
  connection,
  new Wallet(backendKp),
  AnchorProvider.defaultOptions()
);
setProvider(provider);
const program = new Program(Plinko, programId, provider);

export const getClaimTransaction = async (req: Request, res: Response) => {
  const payload = getPayload(req);
  if (!payload) return;

  const { wallet } = payload;
  const { tokenMint, amount } = req.params;
  const claimer = new PublicKey(wallet);
  
  // get claim amount from DB
  const player = await sendRequest(`https://api.servica.io/extorio/apis/blinko`, {
    endpoint: 'getPlayer',
    gameName: 'blinko',
    walletAddress: wallet,
    tokenSPLAddress: tokenMint
  }); 
  if (!player) {
    return res.status(500).json("There's no balance");
  }
  // console.log(player.balance, typeof player.balance);
  
  const claimAmount = !amount ? player.balance : (amount > player.balance ? player.balance : amount);
  const [game] = await getGameAddress(program.programId, game_name, game_owner);
  const gameData = await program.account.game.fetchNullable(game);
  if (!gameData) return res.status(500).json({ error: "Cannot get game data" });
  const mint = new PublicKey(tokenMint);

  const transaction = new Transaction();

  const claimerAta = await getAta(mint, claimer);
  const instruction = await getCreateAtaInstruction(provider, claimerAta, mint, claimer);
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

  // console.log("Transaction made by BE:", transaction);
  transaction.partialSign(backendKp);
  // console.log("Partial Signed Transaction by BE Keypair:", transaction);
  let serializedBuffer = transaction.serialize({ requireAllSignatures: false }).toString("base64");

  return res.json(serializedBuffer);
};

export const sendCalimTransaction = async (req: Request, res: Response) => {
  const payload = getPayload(req);
  if (!payload) return;

  const { wallet } = payload;
  const { tokenMint } = req.params;

  const { serializedBuffer } = req.body;
  const recoveredTx = Transaction.from(Buffer.from(serializedBuffer, "base64"));
  const txSignature = await program.provider.connection.sendRawTransaction(recoveredTx.serialize(), { skipPreflight: true });
  await program.provider.connection.confirmTransaction(txSignature, "confirmed");
  console.log(txSignature);
  // Get claim amount from Transacction
  const programInstructions = recoveredTx.instructions.filter(instruction => instruction.programId.toString() === programId);
  const claimInstruction = programInstructions[0];
  const amountBytes = claimInstruction.data.slice(8).reverse();
  const amount = new BN(amountBytes);
  console.log("Claim amount: ", amount.toString());
  // Decrease wallet's tokeMint amount in DB
  const player = await sendRequest(`https://api.servica.io/extorio/apis/blinko`, {
    endpoint: 'getPlayer',
    gameName: 'blinko',
    walletAddress: wallet,
    tokenSPLAddress: tokenMint
  }); 
  if (!player) {
    return res.status(500).json("There's no balance");
  }
  const balance = player.balance - amount.toNumber() / LAMPORTS_PER_SOL;
  await sendRequest(`https://api.servica.io/extorio/apis/blinko`, {
    endpoint: 'updatePlayer',
    gameName: 'blinko',
    walletAddress: wallet,
    tokenSPLAddress: tokenMint,
    balance
  }); 

  return res.json({ txSignature, claimAmount: amount.toString() })
}

export const sendDepositTransaction = async (req: Request, res: Response) => {
  const payload = getPayload(req);
  if (!payload) return;

  const { wallet } = payload;
  const { tokenMint } = req.params;

  const { serializedBuffer } = req.body;
  const recoveredTx = Transaction.from(Buffer.from(serializedBuffer, "base64"));
  const txSignature = await program.provider.connection.sendRawTransaction(recoveredTx.serialize(), { skipPreflight: false });
  await program.provider.connection.confirmTransaction(txSignature, "confirmed");
  console.log(txSignature);
  // Get deposit amount from Transacction
  const programInstructions = recoveredTx.instructions.filter(instruction => instruction.programId.toString() === programId);
  const claimInstruction = programInstructions[0];
  const amountBytes = claimInstruction.data.slice(8).reverse();
  const amount = new BN(amountBytes);
  console.log("Deposit amount: ", amount.toString());
  // Increase wallet's tokeMint amount in DB
  const player = await sendRequest(`https://api.servica.io/extorio/apis/blinko`, {
    endpoint: 'getPlayer',
    gameName: 'blinko',
    walletAddress: wallet,
    tokenSPLAddress: tokenMint
  }); 
  if (!player) {
    return res.status(500).json("There's no balance");
  }
  const balance = player.balance + amount.toNumber() / LAMPORTS_PER_SOL;
  await sendRequest(`https://api.servica.io/extorio/apis/blinko`, {
    endpoint: 'updatePlayer',
    gameName: 'blinko',
    walletAddress: wallet,
    tokenSPLAddress: tokenMint,
    balance
  }); 

  return res.json({ txSignature, depositAmount: amount.toString() })
}
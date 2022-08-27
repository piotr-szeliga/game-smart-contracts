import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
// @ts-ignore
import { AnchorRaffleTicket } from "../target/types/anchor_raffle_ticket";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  TransactionInstruction
} from '@solana/web3.js';

import {getOrCreateAssociatedTokenAccount, TOKEN_PROGRAM_ID} from "@solana/spl-token";

async function getAndPrintAccount(program: any, raffleAddress: PublicKey)
{
  let account = await program.account.raffle.fetchNullable(raffleAddress);
  if (account) {
    // @ts-ignore
    account.pricePerTicketNum = account.pricePerTicket.toNumber();
    // @ts-ignore
    account.pricePerTicketSOL = account.pricePerTicket.toNumber() / anchor.web3.LAMPORTS_PER_SOL + " Sol";
    // @ts-ignore
    account.tokenType = account.tokenType.toString();
  }
  console.log('Account details:', account);

  return account;
}

async function spawnMoney(
  program: anchor.Program<AnchorRaffleTicket>,
  to: PublicKey,
  sol: number
): Promise<anchor.web3.TransactionSignature> {
  const lamports = sol * anchor.web3.LAMPORTS_PER_SOL;
  const transaction = new anchor.web3.Transaction();
  transaction.add(
      anchor.web3.SystemProgram.transfer({
        // @ts-ignore
        fromPubkey: program.provider.wallet.publicKey,
        lamports,
        toPubkey: to,
      })
  );

  // @ts-ignore
  console.log(`Sending SOL: ${program.provider.wallet.publicKey.toString()} sent ${sol} to ${to.toString()} `);

  const tx = await program.provider.sendAndConfirm(transaction, [], {
    commitment: "confirmed",
  });
  ;

  console.log("DONE:", tx);
  return tx;
}

describe("anchor-game-ticket", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.AnchorRaffleTicket as anchor.Program<AnchorRaffleTicket>;

    const initializedTestActive = false;
    const buyTicketTestActive = true;
    const splTokenTestActive = true;

    it("Program Init!", async () => {
        if (!initializedTestActive) return;

        console.log("\n\nSTART:\n");
        // @ts-ignore
        console.log("Program Wallet:", program.provider.wallet.publicKey.toString());

        // Fund me
        // const airdropSignature = await program.provider.connection.requestAirdrop(
        //     new PublicKey("6rqb63zZ2YgNty5QRz9PLFbN1oPxZdZaP5TwG4vtWHsQ"),
        //     2 * anchor.web3.LAMPORTS_PER_SOL
        // );
        // await new Promise(f => setTimeout(f, 1000));

        // Add your test here.
        //const receiver = Keypair.generate();
        const receiver = Keypair.fromSecretKey(new Uint8Array([133, 230, 105, 82, 126, 147, 188, 49, 144, 121, 98, 112, 160, 239, 106, 142, 105, 92, 58, 193, 34, 169, 161, 57, 57, 231, 154, 146, 19, 17, 244, 172, 16, 123, 70, 229, 190, 105, 161, 60, 53, 123, 148, 82, 214, 237, 122, 193, 24, 62, 101, 168, 243, 70, 149, 117, 33, 159, 75, 104, 193, 83, 97, 231]));
        console.log("receiver:", receiver.publicKey.toString());
        //console.log(receiver.secretKey.toString());
        await spawnMoney(program, receiver.publicKey, 0.1);

        const tokenType = PublicKey.default;
        // const tokenType = new PublicKey("DUSTawucrTsGU8hcqRdHDCbuYhCPADMLM2VcCb8VnFnQ");
        console.log('token type:', tokenType.toString());

        console.log("Step-1 DONE");

        // const raffle = Keypair.generate();
        const raffle = Keypair.fromSecretKey(new Uint8Array([116, 70, 177, 15, 159, 21, 163, 29, 18, 111, 62, 73, 143, 52, 203, 88, 129, 60, 61, 116, 176, 164, 238, 178, 105, 163, 25, 225, 65, 211, 117, 131, 188, 197, 246, 113, 242, 134, 90, 196, 40, 170, 246, 139, 143, 141, 232, 15, 196, 251, 28, 76, 66, 22, 115, 20, 32, 220, 89, 54, 14, 235, 241, 65]));
        console.log("raffle :", raffle.publicKey.toString());
        console.log(raffle.secretKey.toString());

        let account = await getAndPrintAccount(program, raffle.publicKey);

        if (!account) {
            // @ts-ignore
            const price = 0.1;
            const priceBN = new anchor.BN(price * anchor.web3.LAMPORTS_PER_SOL);
            const amount = 8;
            await program.rpc.initialize(tokenType, priceBN, amount,
                {
                    accounts: {
                        payer: receiver.publicKey,
                        raffle: raffle.publicKey,
                        systemProgram: SystemProgram.programId,
                    },
                    signers: [receiver, raffle],
                    options: {
                        commitment: "confirmed"
                    }
                });
        }

        console.log("Step-2 DONE");
        //return;

        const buyer = Keypair.generate();
        console.log("buyer:", buyer.publicKey.toString());
        await spawnMoney(program, buyer.publicKey, 1);

        const ticketsAmountToBuy = 1;
        const ticketPrice = 0.1 * anchor.web3.LAMPORTS_PER_SOL;
        console.log(`wants to buy tickets: ${ticketsAmountToBuy} price: ${ticketPrice}`);

        const listener = program.addEventListener("BuyEvent", (event, slot) => {
            console.log("BuyEvent:", event.buyer.toString(), event.amount, event.soldTickets, event.totalTickets, event.remainingTickets, slot);
        })

        console.log("Step-3 DONE");
        // @ts-ignore
        await program.rpc.buyTicket(ticketsAmountToBuy, new anchor.BN(ticketPrice),
            {
                accounts: {
                    buyer: buyer.publicKey,
                    recipient: receiver.publicKey,
                    raffle: raffle.publicKey,
                    systemProgram: SystemProgram.programId,
                },
                signers: [buyer],
                options: {
                    commitment: "confirmed"
                }
            });

        account = await getAndPrintAccount(program, raffle.publicKey);

        await program.removeEventListener(listener);
    });

    it("Buy Ticket!", async () => {
        if (!buyTicketTestActive) return;

        console.log("\nStep-1:");

        // @ts-ignore
        console.log("Program Wallet:", program.provider.wallet.publicKey.toString());

        //const receiver = Keypair.generate();
        const receiver = Keypair.fromSecretKey(new Uint8Array([133, 230, 105, 82, 126, 147, 188, 49, 144, 121, 98, 112, 160, 239, 106, 142, 105, 92, 58, 193, 34, 169, 161, 57, 57, 231, 154, 146, 19, 17, 244, 172, 16, 123, 70, 229, 190, 105, 161, 60, 53, 123, 148, 82, 214, 237, 122, 193, 24, 62, 101, 168, 243, 70, 149, 117, 33, 159, 75, 104, 193, 83, 97, 231]));
        console.log("receiver:", receiver.publicKey.toString());
        //console.log(receiver.secretKey.toString());
        await spawnMoney(program, receiver.publicKey, 0.1);

        const tokenType = PublicKey.default;
        // const tokenType = new PublicKey("DUSTawucrTsGU8hcqRdHDCbuYhCPADMLM2VcCb8VnFnQ");
        console.log('Token type:', tokenType.toString());

        // const raffle = Keypair.generate();
        const raffle = Keypair.fromSecretKey(new Uint8Array([116, 70, 177, 15, 159, 21, 163, 29, 18, 111, 62, 73, 143, 52, 203, 88, 129, 60, 61, 116, 176, 164, 238, 178, 105, 163, 25, 225, 65, 211, 117, 131, 188, 197, 246, 113, 242, 134, 90, 196, 40, 170, 246, 139, 143, 141, 232, 15, 196, 251, 28, 76, 66, 22, 115, 20, 32, 220, 89, 54, 14, 235, 241, 65]));
        console.log("Raffle :", raffle.publicKey.toString());
        console.log("Raffle Secret:", raffle.secretKey.toString());

        await getAndPrintAccount(program, raffle.publicKey);

        console.log("\nStep-2:");
        const buyer = Keypair.generate();
        console.log("Buyer:", buyer.publicKey.toString());
        await spawnMoney(program, buyer.publicKey, 1);

        const ticketsAmountToBuy = 1;
        const ticketsPrice = 0.1;
        const ticketPriceLAMPORTS = ticketsAmountToBuy * ticketsPrice * anchor.web3.LAMPORTS_PER_SOL;

        console.log("\nStep-3:");
        console.log(`Wants to buy tickets: ${ticketsAmountToBuy} price: ${ticketsPrice}`);

        const listener = program.addEventListener("BuyEvent", (event, slot) => {
            console.log("BuyEvent:", event.buyer.toString(), event.amount, event.soldTickets, event.totalTickets, event.remainingTickets, slot);
        })

        console.log("\nStep-4 Buying...");

        // @ts-ignore
        await program.rpc.buyTicketSol(ticketsAmountToBuy, new anchor.BN(ticketPriceLAMPORTS),
            {
                accounts: {
                    buyer: buyer.publicKey,
                    recipient: receiver.publicKey,
                    raffle: raffle.publicKey,
                    systemProgram: SystemProgram.programId,
                },
                signers: [buyer],
                options: {
                    commitment: "confirmed"
                }
            });

        let account = await getAndPrintAccount(program, raffle.publicKey);

        await program.removeEventListener(listener);
    });

    it("Spl Token!", async () => {
        if (!splTokenTestActive) return;

        const mint = new PublicKey("ASxC3n3smkcUkA7Z58EUKZ2NfHoQ8eZrkTRK7ergYr2a"); // $CRECK devnet
        const sender = program.provider.publicKey;
        console.log("Sender:", sender.toString());
        // @ts-ignore
        const senderWallet = program.provider.wallet;
        const recipient = new PublicKey("3xeW8eLMunbmMW83n2wLqNkiEr4GsUFJjzM6h19fhwot"); // raffle bank
        const senderATA = await getOrCreateAssociatedTokenAccount(program.provider.connection, senderWallet.payer, mint, sender);
        console.log(senderATA.address.toString());

        const recipientATA = await getOrCreateAssociatedTokenAccount(program.provider.connection, senderWallet.payer, mint, recipient);
        console.log(recipientATA.address.toString());

        const ticketsAmountToBuy = 3;
        const ticketsPrice = 0.1;
        const ticketPriceLAMPORTS = ticketsAmountToBuy * ticketsPrice * anchor.web3.LAMPORTS_PER_SOL;

        // @ts-ignore
        await program.rpc.buyTicketSpl(ticketsAmountToBuy, new anchor.BN(ticketPriceLAMPORTS),
            {
                accounts: {
                    sender: sender,
                    senderTokens: senderATA.address,
                    recipientTokens: recipientATA.address,
                    tokenProgram: TOKEN_PROGRAM_ID
                }
            });
    });
});

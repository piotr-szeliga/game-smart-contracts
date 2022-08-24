import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
// @ts-ignore
import { AnchorRaffleTicket } from "../target/types/anchor_raffle_ticket";
import {
  Keypair,
  PublicKey,
  SystemProgram
} from '@solana/web3.js';

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

  return await program.provider.sendAndConfirm(transaction, [], {
    commitment: "confirmed",
  });
}

describe("anchor-game-ticket", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorRaffleTicket as Program<AnchorRaffleTicket>;

  it("Is initialized!", async () =>
  {
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
    const receiver = Keypair.fromSecretKey(new Uint8Array([133,230,105,82,126,147,188,49,144,121,98,112,160,239,106,142,105,92,58,193,34,169,161,57,57,231,154,146,19,17,244,172,16,123,70,229,190,105,161,60,53,123,148,82,214,237,122,193,24,62,101,168,243,70,149,117,33,159,75,104,193,83,97,231]));
    console.log("receiver:", receiver.publicKey.toString());
    //console.log(receiver.secretKey.toString());
    await spawnMoney(program, receiver.publicKey, 0.1);

    console.log("Step-1 DONE");

    // const raffle = Keypair.generate();
    const raffle = Keypair.fromSecretKey(new Uint8Array([159,238,176,226,53,7,139,231,65,16,97,112,122,138,104,181,255,203,13,132,68,226,195,28,103,159,236,14,230,40,149,88,18,176,59,241,187,184,125,217,91,112,78,227,125,55,22,153,76,159,3,170,89,221,76,214,152,23,169,204,50,216,31,87]));
    console.log("raffle :", raffle.publicKey.toString());
    console.log(raffle.secretKey.toString());

    const account = await program.account.raffle.fetchNullable(raffle.publicKey);
    // @ts-ignore
    account.pricePerTicketSOL = account.pricePerTicket.toNumber() / anchor.web3.LAMPORTS_PER_SOL + " Sol";
    console.log('account exists:', account);

    if (!account)
    {
      // @ts-ignore

      const price = 3.2;
      await program.rpc.initialize(new anchor.BN(price * anchor.web3.LAMPORTS_PER_SOL), 7,
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
    return;

    const buyer = Keypair.generate();
    console.log("buyer: ", buyer.publicKey.toString());
    await spawnMoney(program, buyer.publicKey, .2);

    console.log("Step-3");
    const listener = program.addEventListener("BuyEvent", (event, slot) => {
      console.log("BuyEvent:", event.buyer.toString(), event.amount, event.soldTickets, event.totalTickets, event.remainingTickets, slot);
    })

    // @ts-ignore
    await program.rpc.buyTicket(1, {
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

    await new Promise(f => setTimeout(f, 5000));

    await program.removeEventListener(listener);
  });
});

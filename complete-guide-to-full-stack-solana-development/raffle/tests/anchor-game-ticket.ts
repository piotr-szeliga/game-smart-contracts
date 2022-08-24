import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
// @ts-ignore
import { AnchorGameTicket } from "../target/types/anchor_game_ticket";
import {
  Keypair,
  PublicKey,
  SystemProgram
} from '@solana/web3.js';

async function spawnMoney(
  program: anchor.Program<AnchorGameTicket>,
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
  return await program.provider.sendAndConfirm(transaction, [], {
    commitment: "confirmed",
  });
}

describe("anchor-game-ticket", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorGameTicket as Program<AnchorGameTicket>;

  it("Is initialized!", async () =>
  {
    console.log("START");
    // @ts-ignore
    console.log("Program Wallet: ", program.provider.wallet.publicKey.toString());
    // Fund me
    // const airdropSignature = await program.provider.connection.requestAirdrop(
    //     new PublicKey("6rqb63zZ2YgNty5QRz9PLFbN1oPxZdZaP5TwG4vtWHsQ"),
    //     2 * anchor.web3.LAMPORTS_PER_SOL
    // );
    // await new Promise(f => setTimeout(f, 1000));

    // Add your test here.
    const receiver = Keypair.generate();
    console.log("receiver: ", receiver.publicKey.toString());
    await spawnMoney(program, receiver.publicKey, 0.05);
    console.log("Step-1");

    //const raffle = Keypair.generate();
    const raffle = Keypair.fromSecretKey(new Uint8Array([124,211,103,193,156,71,237,41,91,34,3,54,153,166,248,82,235,87,57,59,157,116,252,83,224,65,202,118,152,6,228,75,209,212,209,22,216,40,227,169,105,1,33,119,247,42,110,129,130,187,35,115,231,56,130,83,2,155,213,235,40,91,92,40]));
    console.log("raffle :", raffle.publicKey.toString());
    console.log(raffle.secretKey.toString());

    const account = await program.account.game.fetchNullable(raffle.publicKey);
    console.log('account exists: ', account);

    if (!account)
    {
      // @ts-ignore
      await program.rpc.initialize(5, {
        accounts: {
          payer: receiver.publicKey,
          game: raffle.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [receiver, raffle],
        options: {
          commitment: "confirmed"
        }
      });
    }

    console.log("Step-2");
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
        game: raffle.publicKey,
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

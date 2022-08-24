import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
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

  it("Is initialized!", async () => {
    // Add your test here.
    const payer = Keypair.generate();
    await spawnMoney(program, payer.publicKey, 1);

    const game = Keypair.generate();

    await program.rpc.initialize(1000, {
      accounts: {
        payer: payer.publicKey,
        game: game.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [payer, game],
      options: {
        commitment: "confirmed"
      }
    });

    const buyer = Keypair.generate();
    await spawnMoney(program, buyer.publicKey, 15);

    const listener = program.addEventListener("BuyEvent", (event, slot) => {
      console.log(event.buyer, event.amount, slot)
    })

    await program.rpc.buyTicket(10, {
      accounts: {
        buyer: buyer.publicKey,
        recipient: payer.publicKey,
        game: game.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [buyer],
      options: {
        commitment: "confirmed"
      }
    });

    await program.removeEventListener(listener);
  });
});

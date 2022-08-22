import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MySolData } from "../target/types/my_sol_data";

describe("my_sol_data", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MySolData as Program<MySolData>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

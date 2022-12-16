import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SubflowSol } from "../target/types/subflow_sol";

describe("subflow_sol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SubflowSol as Program<SubflowSol>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AtaStakingV3 } from "../target/types/ata_staking_v3";

describe("ata-staking-v3", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AtaStakingV3 as Program<AtaStakingV3>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

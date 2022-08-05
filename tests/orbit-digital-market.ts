import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { OrbitDigitalMarket } from "../target/types/orbit_digital_market";

describe("orbit-digital-market", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.OrbitDigitalMarket as Program<OrbitDigitalMarket>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});

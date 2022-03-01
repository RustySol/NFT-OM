import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { LiqnftMarket } from '../target/types/liqnft_market';

describe('liqnft-market', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.LiqnftMarket as Program<LiqnftMarket>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});

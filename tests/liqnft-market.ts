import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { LiqnftMarket } from '../target/types/liqnft_market';
import { AUCTION_HOUSE_PROGRAM_ID, WRAPPED_SOL_MINT } from './constants';
import { getAuctionHouse, getAuctionHouseAuthority, getAuctionHouseFeeAcct, getAuctionHouseTreasuryAcct } from './utils';

describe('liqnft-market', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.LiqnftMarket as Program<LiqnftMarket>;

  xit('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });

  it("Create auction house via proxy", async () => {
    const walletKeyPair = program.provider.wallet;

    const sfbp = 200;

    const tMintKey = WRAPPED_SOL_MINT;

    const [auctionHouse, bump] = await getAuctionHouse(
      walletKeyPair.publicKey,
      tMintKey
    );


    const [auctionHouseAuthority, authorityBump] =
      await getAuctionHouseAuthority(walletKeyPair.publicKey, tMintKey);


      const [feeAccount, feeBump] = await getAuctionHouseFeeAcct(auctionHouseAuthority);
      const [treasuryAccount, treasuryBump] = await getAuctionHouseTreasuryAcct(
        auctionHouseAuthority
      );

    console.log("tMintKey ", tMintKey.toString())
    console.log("walletKeyPair ", walletKeyPair.publicKey.toString())
    console.log("authority ", auctionHouseAuthority.toString())
    console.log("auctionHouse ", auctionHouse.toString())
    console.log("feeAccount ", feeAccount.toString())
    console.log("treasuryAccount ", treasuryAccount.toString())

    await program.rpc.createAuctionHouseWithProxy(
      bump,
      feeBump,
      treasuryBump,
      sfbp,
      "true",
      "true",
      {
        accounts: {
          treasuryMint: tMintKey,
          payer: walletKeyPair.publicKey,
          authority: auctionHouseAuthority,
          feeWithdrawalDestination: walletKeyPair.publicKey,
          treasuryWithdrawalDestination: walletKeyPair.publicKey,
          treasuryWithdrawalDestinationOwner: walletKeyPair.publicKey,
          auctionHouse,
          auctionHouseFeeAccount: feeAccount,
          auctionHouseTreasury: treasuryAccount,
          ahProgram: AUCTION_HOUSE_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
        },
      }
    );
  });
});

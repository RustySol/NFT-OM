import { PublicKey } from "@solana/web3.js";
import {
  AUCTION_HOUSE,
  AUCTION_HOUSE_PROGRAM_ID,
  FEE_PAYER,
  LIQNFT_MARKET,
  TREASURY,
} from "./constants";

export const getAuctionHouse = async (
  creator: PublicKey,
  treasuryMint: PublicKey
): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [Buffer.from(AUCTION_HOUSE), creator.toBuffer(), treasuryMint.toBuffer()],
    AUCTION_HOUSE_PROGRAM_ID
  );
};



export const getAuctionHouseAuthority = async (
  creator: PublicKey,
  treasuryMint: PublicKey
): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [Buffer.from(AUCTION_HOUSE), creator.toBuffer(), treasuryMint.toBuffer(), Buffer.from(LIQNFT_MARKET)],
    AUCTION_HOUSE_PROGRAM_ID
  );
};


export const getAuctionHouseProgramAsSigner = async (): Promise<
  [PublicKey, number]
> => {
  return await PublicKey.findProgramAddress(
    [Buffer.from(AUCTION_HOUSE), Buffer.from("signer")],
    AUCTION_HOUSE_PROGRAM_ID
  );
};

export const getAuctionHouseFeeAcct = async (
  auctionHouse: PublicKey
): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [
      Buffer.from(AUCTION_HOUSE),
      auctionHouse.toBuffer(),
      Buffer.from(FEE_PAYER),
    ],
    AUCTION_HOUSE_PROGRAM_ID
  );
};

export const getAuctionHouseTreasuryAcct = async (
  auctionHouse: PublicKey
): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [
      Buffer.from(AUCTION_HOUSE),
      auctionHouse.toBuffer(),
      Buffer.from(TREASURY),
    ],
    AUCTION_HOUSE_PROGRAM_ID
  );
};

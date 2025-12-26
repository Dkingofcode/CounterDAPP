// Here we export some useful types and functions for interacting with the Anchor program.
import { Cluster, PublicKey } from '@solana/web3.js';
import type { SolanaDappCounterFrontend } from '../target/types/solana_dapp_counter_frontend';

import idl from  '../target/idl/solana_dapp_counter_frontend.json';
//const idl: any =  require("../target/idl/solana_dapp_counter_frontend.json");

// Re-export the generated IDL and type
 export { SolanaDappCounterFrontend }; 


// ✅ Export IDL
export const SolanaDappCounterFrontendIDL = idl as any;

// After updating your program ID (e.g. after running `anchor keys sync`) update the value below.
export const SOLANA_DAPP_COUNTER_FRONTEND_PROGRAM_ID = new PublicKey(
  'G3eJNm4Kwgf13zESTtES6DYFAkgsknBmpJsKRGJay2DN'
);

// This is a helper function to get the program ID for the SolanaDappCounterFrontend program depending on the cluster.
export function getSolanaDappCounterFrontendProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
    case 'mainnet-beta':
    default:
      return SOLANA_DAPP_COUNTER_FRONTEND_PROGRAM_ID;
  }
}

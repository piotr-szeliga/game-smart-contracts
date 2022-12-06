import express from "./config/express";
import { getSettings, setSettings, getAdminSettings } from "./controller/setting.controller";
import { getClaimTransaction, sendCalimTransaction, sendDepositTransaction } from "./controller/transaction.controller";
import { authorized, isAdmin } from "./middleware/auth.middleware";
// const { Keypair } = require('@solana/web3.js');
// const bs58 = require('bs58');

express.post('/transaction/deposit/:clientKey', authorized, sendDepositTransaction);
express.get('/transaction/claim/:clientKey', getClaimTransaction);
express.post('/transaction/claim/:clientKey', authorized, sendCalimTransaction);
express.get('/settings/admin', authorized, isAdmin, getAdminSettings);
express.get('/settings', getSettings);
express.post('/settings', authorized, isAdmin, setSettings);

// const kp = Keypair.generate();
// console.log(kp.publicKey.toString());
// console.log(bs58.encode(kp.secretKey));

if (process.env.NODE_ENV === "development") {
  express.listen(express.get("port"), () => {
    console.log(
      `Server running at http://${express.get("host")}:${express.get("port")}`
    );
  });
} else {
  //
}

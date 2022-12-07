import express from "./config/express";
import { getPlayStatus } from "./controller/game.controller";
import { getSettings, setSettings, getAdminSettings } from "./controller/setting.controller";
import { getClaimTransaction, sendCalimTransaction, sendDepositTransaction } from "./controller/transaction.controller";
import { authorizedAdmin, authorizedPlayer } from "./middleware/auth.middleware";
// const { Keypair } = require('@solana/web3.js');
// const bs58 = require('bs58');

express.post('/transaction/deposit/:clientKey', authorizedPlayer, sendDepositTransaction);
express.get('/transaction/claim/:clientKey', authorizedPlayer, getClaimTransaction);
express.post('/transaction/claim/:clientKey', authorizedPlayer, sendCalimTransaction);
express.get('/settings/admin', authorizedAdmin, getAdminSettings);
express.post('/settings/admin', authorizedAdmin, setSettings);
express.get('/settings', authorizedPlayer, getSettings);
express.post('/game', authorizedPlayer, getPlayStatus);

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

import express from "./config/express";
import { getClaimTransaction, sendCalimTransaction, sendDepositTransaction } from "./controller/transaction.controller";
// const { Keypair } = require('@solana/web3.js');
// const bs58 = require('bs58');

express.post('/transaction/deposit/:clientKey', sendDepositTransaction);
express.get('/transaction/claim/:clientKey', getClaimTransaction);
express.post('/transaction/claim/:clientKey', sendCalimTransaction);


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

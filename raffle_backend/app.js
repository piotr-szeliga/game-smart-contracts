const app = require("./config/express");
const txCtrl = require("./controller/transaction.controller");
const { Keypair, PublicKey } = require("@solana/web3.js");
const bs58 = require("bs58");
const anchor = require("@project-serum/anchor");
const {
  loadSwitchboardProgram,
  VrfAccount,
} = require("@switchboard-xyz/switchboard-v2");

let payer = Keypair.fromSecretKey(
  bs58.decode("")
);

const program = await loadSwitchboardProgram("devnet", undefined, payer);

const vrfKey = new PublicKey("");
const vrfAccount = new VrfAccount({
  program,
  publicKey: vrfKey,
});
const vrf = await vrfAccount.loadData();
console.log(vrf.currentRound.result);

app.get("/transaction/:clientKey", txCtrl.getTransaction);

// const kp = Keypair.generate();
// console.log(kp.publicKey.toString());
// console.log(bs58.encode(kp.secretKey));

if (process.env.NODE_ENV === "development") {
  app.listen(app.get("port"), () => {
    console.log(
      `Server running at http://${app.get("host")}:${app.get("port")}`
    );
  });
} else {
  //
}

var anchor = require("@project-serum/anchor");
var idl = require("./contract/idl/anchor_game_ticket.json");
var web3 = require("@solana/web3.js");
var bs58 = require("bs58");
const fs = require("fs");
const {
  clusterApiUrl,
  Connection,
  Keypair,
  Transaction,
  SystemProgram,
  PublicKey,
} = web3;

let output;

let i = 0;
const keypair = web3.Keypair.fromSecretKey(
  bs58.decode(
    "533KhRG28nh5LgmC9ZtXNYdaUGjWvxddzHLPBRny1Uha3xUsab8bKpz9ZRoKREMTv3XP4VaKdWfrJXj53A4G18dt"
  )
);
const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
const provider = new anchor.AnchorProvider(connection, keypair);
const program = new anchor.Program(idl, idl.metadata.address, provider);
const listener = program.addEventListener("BuyEvent", (event, slot) => {
  console.log(event, slot);
  checkUser(event.buyer.toString(), event.amount);
});

const fechData = async (address) => {
  const data = await program.account.game.fetchNullable(
    new PublicKey(address)
  );
  console.log(data, data.totalTickets, data.soldTickets);
};

const writeFile = async (jsonContent) => {
  await fs.writeFile("output.json", jsonContent, "utf8", function (err) {
    if (err) {
      console.log("An error occured while writing JSON Object to File.");
      return console.log(err);
    }

    console.log("JSON file has been saved.");
  });
};

const checkUser = (address, amount) => {
  let result = output;
  let len = result.length;
  let i = 0;
  for (i = 0; i < len; i++) {
    if (result[i].buyer == address) {
      result[i].amount += amount;
      break;
    }
  }
  if (i == len) result = [...result, { buyer: address, amount: amount }];
  writeFile(JSON.stringify(result));
};

const path = "./output.json";

try {
  if (!fs.existsSync(path)) {
    let temp = new Array();
    writeFile(JSON.stringify(temp));
  }
  output = require("./output.json");
} catch (err) {
  console.error(err);
}

fechData("7ijco6QKXiHfnDo76dp7RYPCeu9Yx4x1iBVsZ7j9XsS7");

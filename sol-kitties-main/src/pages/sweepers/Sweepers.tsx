import React, { FC, useEffect, useState, ChangeEvent } from "react";
import { ToastContainer, toast } from "react-toastify";
import { useWalletModal } from "@solana/wallet-adapter-react-ui";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import {
  Connection,
  PublicKey,
  clusterApiUrl,
  SystemProgram,
  Keypair,
} from "@solana/web3.js";
import {
  Program,
  AnchorProvider,
  web3,
  setProvider,
} from "@project-serum/anchor";
import Countdown from "react-countdown";

import Connect2Phantom from "components/Connect2Phantom";
import Avatararea from "./sections/Avatararea";
import FAQ from "./sections/FAQ";
import Connect2PhantomBtn from "components/Connect2PhantomBtn";

import idl from "contract/idl/anchor_game_ticket.json";

const Sweepers = () => {
  // Front end state variable
  const [value, setValue] = React.useState(1); // Input chage state variable
  const [ticketValue, setTicketValue] = React.useState(0.15); // one ticket state variable
  const [totalValue, setTotalValue] = React.useState(0.15); // total ticket state variable
  const [soldTicket, setSoldTicket] = React.useState("0");

  // Wallet Connect state variable
  const wallet = useWallet(); // wallet connection state variable
  const { setVisible } = useWalletModal(); // Modal state variable
  const [connected, setConnected] = useState(false); // Flag state variable for current public key connecting state
  const [active, setActive] = useState(false); // Flag state variable for Select wallet or Connect???

  const { connection } = useConnection();
  //@ts-ignore
  const provider = new AnchorProvider(connection, wallet);
  setProvider(provider);
  const program = new Program(
    //@ts-ignore
    idl,
    idl.metadata.address,
    provider
  );

  useEffect(() => {
    program.account.game
      .fetchNullable(
        new PublicKey("7ijco6QKXiHfnDo76dp7RYPCeu9Yx4x1iBVsZ7j9XsS7")
      )
      .then((result: any) => {
        setSoldTicket(result.soldTickets.toString());
      });
  });

  //InPut change event
  const handleInputChange = (e: ChangeEvent<HTMLInputElement>) => {
    setValue(parseInt(e.target.value));
  };

  // total ticket change event according to value change
  useEffect(() => {
    if (value < 1) setValue(1);
    setTotalValue(ticketValue * value);
  }, [value, ticketValue]);

  // Input plus event
  const plus = async () => {
    await setValue(value + 1);
  };

  // Input Minus event
  const minus = async () => {
    if (value < 1) toast.warn("Ticket must be at least 1!!");
    let temp = value - 1 < 1 ? 1 : value - 1;
    await setValue(temp);
  };

  // React Counter
  // @ts-ignore
  const renderer = ({ days, hours, minutes, seconds, completed }) => {
    if (completed) {
      // Render a complete state
      // return <Completionist />;
    } else {
      return (
        <p className="text-xl font-black text-[#fff] mt-3">
          {days}d:{hours}h:{minutes}m:{seconds}s
        </p>
      );
    }
  };

  // select wallet Modal function
  const openModal = async () => {
    setVisible(true);
  };

  // currenct selected wallet useEffect
  useEffect(() => {
    wallet.wallet ? setActive(true) : setActive(false);
  }, [wallet.wallet]);

  // currenct selected publickey useEffect
  useEffect(() => {
    wallet.publicKey ? setConnected(true) : setConnected(false);
  }, [wallet.publicKey]);

  // wallet connection function
  const useConnect = () => {
    wallet.connect();
  };
  // wallet disconnect function
  const useDisconnect = () => {
    wallet.disconnect();
  };

  // buty ticket function
  const buyTicket = async () => {
    try {
      const game = new PublicKey(
        "7ijco6QKXiHfnDo76dp7RYPCeu9Yx4x1iBVsZ7j9XsS7"
      );
      const recipient = new PublicKey(
        "3HeV96b8euHLxifCWp63YH6A4ZiVkrxjN8LcswCJkXzr"
      );
      //@ts-ignore
      const tx = await program.transaction.buyTicket(value, {
        accounts: {
          buyer: wallet.publicKey,
          recipient,
          game,
          systemProgram: SystemProgram.programId,
        },
      });
      const txSignagture = await wallet.sendTransaction(
        tx,
        program.provider.connection
      );
      program.provider.connection
        .confirmTransaction(txSignagture, "confirmed")
        .then((result) => {
          if (result) {
            toast.success(`${totalValue} sol was successfully transferred!`);
            program.account.game.fetchNullable(game).then((result: any) => {
              setSoldTicket(result.soldTickets.toString());
            });
          }
        })
        .catch((err) => {
          console.log(err);
          toast.warn(`Your balance is not enough!`);
        });

      // console.log("Got the account", account);
    } catch (error) {
      console.log("Error in getGifList: ", error);
    }
  };

  const init = async () => {
    const game = Keypair.generate();
    console.log(wallet.publicKey?.toString());
    //@ts-ignore
    const tx = program.transaction.initialize(1000, {
      accounts: {
        payer: wallet.publicKey,
        game: game.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });
    const txSignagture = await wallet.sendTransaction(
      tx,
      program.provider.connection,
      {
        signers: [game],
      }
    );
    await program.provider.connection.confirmTransaction(
      txSignagture,
      "confirmed"
    );
    console.log(game.publicKey.toString());
  };

  return (
    <div className="sweepers-back pt-[25px]">
      <div className="main-layout">
        <ToastContainer />
        <div className="main-container">
          <div className="flex w-full flex-wrap gap-5 justify-center md:justify-between ">
            <div>
              <div className="relative z-50 text-center">
                <img
                  src="images/sweepers-title.png"
                  alt="sweepers title"
                  className="w-[480px] h-full -mt-[25px]"
                />
              </div>
            </div>
            <Connect2Phantom
              walletAvail={active}
              connected={connected}
              publicKey={wallet.publicKey}
              openModal={openModal}
              connectHandler={useConnect}
              disconnectHandler={useDisconnect}
            />
          </div>

          <div className="grid justify-center flex-wrap lg:grid-cols-2 mb-[30px] relative prize-border-line lg:-top-[13px] gap-4 overflow-hidden xl:overflow-visible">
            <div className="lucky-winner-left sm:p-[10%] md:p-0 -mt-[60px] md:mt-0 relative xl:mr-14 overflow-hidden xl:overflow-visible">
              <img
                src="/images/money_dudle.png"
                alt="money_dudle"
                className="relative z-40 md:w-[622px] xl:h-[504px]"
              />
              <div className="left-btn h-[113px] -mt-[10%] md:-mt-[14%] mx-auto p-[5px] relative text-center w-[229px] z-50">
                <label className="uppercase sweep-label flex justify-center mt-[3px] text-[24px] font-black">
                  Sweeping In
                </label>
                <Countdown date={Date.now() + 483827393} renderer={renderer} />
              </div>
            </div>

            <div className="flex items-center justify-center lg:justify-start relative w-full h-full text-center mt-5 md:mt-[80px] lg:right-[32px] xl:right-0 overflow-visible">
              <div className="relative ">
                <div className="right-ray">
                  <p className="text-[#fff] text-2xl font-black absolute -top-[40px] z-50 m-0 left-0 right-0 text-center">
                    Tickets Sold:
                    <span className="bg-[#fff] rounded-[5px] p-[3px] text-[#043465] ml-[5px]">
                      {soldTicket}
                    </span>
                  </p>
                  <img
                    src="images/ticket.gif"
                    alt="ticket"
                    className="mx-auto z-50 w-[230px] relative mt-1"
                  />
                  <img
                    src="images/price_label.png"
                    alt="price_label"
                    className="relative z-50 mt-[20px] mx-auto w-[320px] md:w-[380px] xl:w-[435px] h-full"
                  />
                </div>
                <div className="flex flex-col md:flex-row items-center justify-center gap-[10px]">
                  <div className="flex flex-col text-[#fff] text-[25px] font-bold ">
                    <span>Price</span>
                    <button className="flex items-center justify-center bg-[#120d28] rounded-[10px] w-[125px] h-[37px] border-[1px] border-[#ffd029]">
                      <img
                        src="images/ring.png"
                        alt="ring"
                        className="w-[18px] h-[18px] mr-[5px]"
                      />
                      {totalValue.toFixed(2)}
                    </button>
                  </div>
                  <div className=" cursor-pointer">
                    <Connect2PhantomBtn
                      walletAvail={active}
                      connected={connected}
                      openModal={openModal}
                      connectHandler={useConnect}
                      buyTicket={buyTicket}
                    />
                  </div>
                  <div className="flex flex-col text-[#fff] text-[25px] font-bold">
                    <span>Amount</span>
                    <div className="flex items-center relative">
                      <span
                        className="text-lg absolute w-9 cursor-pointer"
                        onClick={minus}
                      >
                        -
                      </span>
                      <input
                        className="font-bold flex text-center bg-[#120d28] border-[1px] border-[#998feb] w-[125px] rounded-[10px] px-5"
                        value={value}
                        onChange={handleInputChange}
                        pattern="[0-9]*"
                        type="text"
                      />
                      <span
                        className="text-lg absolute w-9 right-0 cursor-pointer"
                        onClick={plus}
                      >
                        +
                      </span>
                    </div>
                  </div>
                </div>
                {/* <button onClick={init} className="text-[#fff]">
                  Init
                </button> */}
                <div className="text-[#86a2ec] font-black text-2xl mb-[10px] mt-[15px] ">
                  Kitties Family Sweeps Together
                </div>
              </div>
            </div>
          </div>

          <FAQ />
        </div>
      </div>
      <Avatararea />
    </div>
  );
};

export default Sweepers;

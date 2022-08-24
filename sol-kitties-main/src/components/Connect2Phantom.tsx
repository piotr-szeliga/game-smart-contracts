import { FC } from "react";
import { PublicKey } from "@solana/web3.js";

type Props = {
  walletAvail?: boolean;
  connected?: boolean;
  publicKey?: PublicKey | null;
  openModal(arg: any): void;
  connectHandler(arg: any): void;
  disconnectHandler(arg: any): void;
};

const Connect2Phantom: FC<Props> = ({
  walletAvail,
  connected,
  publicKey,
  openModal,
  connectHandler,
  disconnectHandler,
}) => {
  return (
    <div className="md:ml-auto">
      <div className="flex flex-col items-end">
        {walletAvail && (
          <button
            className="border-2 rounded-[16px] cursor-pointer flex items-center justify-center bg-[#04386c] p-[5px]"
            onClick={connectHandler}
            // disabled={connected}
            style={
              !connected
                ? { borderColor: "#47deff" }
                : { borderColor: "#ffc800" }
            }
          >
            <img
              src="images/useravatar.png"
              alt="useravatar"
              className="w-[41px] h-[41px] border-r-2"
              style={
                !connected
                  ? { borderColor: "#47deff" }
                  : { borderColor: "#ffc800" }
              }
            />
            <p className="uppercase font-bold text-lg leading-[1.1]  text-[#fff] text-center overflow-hidden px-[7px] w-[145px]">
              {connected ? (
                <>{publicKey?.toBase58()}</>
              ) : (
                <>
                  connect <br /> wallet
                </>
              )}
            </p>
            {connected ? (
              <>
                <div
                  onClick={disconnectHandler}
                  className="cursor-pointer min-w-[25px] w-[25px] h-full mx-2"
                >
                  <img src="images/disconnect.png" alt="disconnect" />
                </div>
              </>
            ) : (
              <></>
            )}
          </button>
        )}
        {!walletAvail && (
          <button
            className="border-2 rounded-[16px] cursor-pointer flex items-center justify-center bg-[#04386c] p-[5px] border-[#47deff]"
            onClick={openModal}
          >
            <img
              src="images/useravatar.png"
              alt="useravatar"
              className="w-[41px] h-[41px] border-r-2 border-[#47deff]"
            />
            <p className="uppercase font-bold text-lg leading-[1.1]  text-[#fff] text-center overflow-hidden px-[7px] w-[145px]">
              select <br /> wallet
            </p>
          </button>
        )}
      </div>
    </div>
  );
};

export default Connect2Phantom;

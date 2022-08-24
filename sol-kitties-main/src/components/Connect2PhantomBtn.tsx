import { FC } from "react";

type Props = {
  connected?: boolean;
  walletAvail?: boolean;
  connectHandler(arg: any): void;
  openModal(arg: any): void;
  buyTicket(): void;
};

const Connect2PhantomBtn: FC<Props> = ({
  connected,
  walletAvail,
  connectHandler,
  openModal,
  buyTicket,
}) => {
  return (
    <>
      {connected && (
        <button
          className="w-[222px] h-[48px] mt-[25px] text-center lottery-btn border-b-[5px] border-b-[#dc771a] rounded-[8px] text-[#62370f] text-[27px] font-black cursor-pointer"
          onClick={buyTicket}
        >
          Buy Tickets
        </button>
      )}
      {!connected && walletAvail && (
        <button
          className="w-[222px] h-[48px] mt-[25px] text-center lottery-btn border-b-[5px] border-b-[#dc771a] rounded-[8px] text-[#62370f] text-[27px] font-black cursor-pointer"
          onClick={connectHandler}
        >
          Connect
        </button>
      )}
      {!walletAvail && (
        <button
          className="w-[222px] h-[48px] mt-[25px] text-center lottery-btn border-b-[5px] border-b-[#dc771a] rounded-[8px] text-[#62370f] text-[27px] font-black cursor-pointer"
          onClick={openModal}
        >
          Connect
        </button>
      )}
    </>
  );
};

export default Connect2PhantomBtn;

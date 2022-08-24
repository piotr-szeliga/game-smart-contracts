import React, { useRef, useEffect } from "react";

type Props = {
  sideOpen: boolean;
  close(arg: boolean): void;
};

const useOutsideAlerter = (ref: any, close: any) => {
  useEffect(() => {
    function handleClickOutside(event: any) {
      if (ref.current && !ref.current.contains(event.target)) {
        close(false);
      }
    }

    document.addEventListener("mousedown", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [ref, close]);
};

const Sidebar: React.FC<Props> = ({ sideOpen, close }) => {
  const wrapperRef = useRef(null);
  useOutsideAlerter(wrapperRef, close);

  return (
    <div>
      <div
        className="flex flex-col bg-[#fff] z-[999] top-0 left-0 w-3/4 h-screen fixed px-4 py-2 transition-all duration-150"
        style={!sideOpen ? { left: "-9999px" } : { left: "0px" }}
        ref={wrapperRef}
      >
        <div className="flex gap-5 items-center">
          <img
            src="images/logo.png"
            alt="logo"
            className="w-[52px] h-[52px] my-[10px] bg-[#fd6] rounded-[10px]"
          />
          <span className="uppercase text-base lg:text-[22px] xl:text-[27px] xl:leading-[31px] cursor-pointer font-bold">
            sol kitties
          </span>
        </div>
        <div className="flex flex-col">
          <p className="py-3 pr-4 text-[#656565] text-base transition-all duration-150 hover:text-[#ffd029] cursor-pointer">
            Verify
          </p>
          <p className="py-3 pr-4 text-[#656565] text-base transition-all duration-150 hover:text-[#ffd029] cursor-pointer">
            Staking
          </p>
          <p className="py-3 pr-4 text-[#656565] text-base transition-all duration-150 hover:text-[#ffd029] cursor-pointer">
            Verify
          </p>
          <p className="py-3 pr-4 text-[#656565] text-base transition-all duration-150 hover:text-[#ffd029] cursor-pointer">
            Sweepers
          </p>
          <p className="py-3 pr-4 text-[#656565] text-base transition-all duration-150 hover:text-[#ffd029] cursor-pointer">
            Roadmap
          </p>
          <p className="py-3 pr-4 text-[#656565] text-base transition-all duration-150 hover:text-[#ffd029] cursor-pointer">
            Faq
          </p>
          <p className="py-3 pr-4 text-[#656565] text-base transition-all duration-150 hover:text-[#ffd029] cursor-pointer">
            Whitepaper
          </p>
        </div>
      </div>
      {sideOpen ? <div className="overlay backdrop-blur-md"></div> : <></>}
    </div>
  );
};

export default Sidebar;

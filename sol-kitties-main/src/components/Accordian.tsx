import React, { useState } from "react";

type Props = {
  question: string;
  answer: string;
};

const Accordian: React.FC<Props> = ({ question, answer }) => {
  const [flag, setFlag] = useState<boolean>(true);

  const rotate = {
    transform: flag ? 'rotate(180deg)' : '', 
    transition: 'transform 150ms ease', // smooth transition
   }

  const toggle = () => {
    setFlag(!flag);
  };

  return (
    <div
      className={`${flag ? 'h-[50px]' : 'h-[155px] md:h-[125px] lg:h-[100px]'} bg-[#243562] flex flex-col rounded-[15px] transition-all duration-300 overflow-hidden cursor-pointer`}
      // style={flag ? { height: "50px" } : { minHeight: "100px" }}
    >
      <div className="flex px-4 py-3 items-center" onClick={toggle}>
        <p className="text-[#fff] text-[17px] font-semibold text-left">
          <span>Q: </span>
          {question}
        </p>
        <img
          src="images/down-arrow.png"
          alt="down-arrow"
          className="w-[22px] h-[20px] ml-auto"
          style={rotate}
        />
      </div>
      <div className="flex px-4 py-3 items-center">
        <p className="text-[#fff] text-[17px] font-light text-left">
          <span className="font-semibold">A: </span>
          {answer}
        </p>
      </div>
    </div>
  );
};

export default Accordian;

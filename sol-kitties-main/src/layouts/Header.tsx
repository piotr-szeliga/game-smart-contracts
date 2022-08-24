import React from "react";

interface FuncProps {
  toggle(arg: void): void;
}

const Header:React.FC<FuncProps> = (props) => {

  return (
    <div className="flex fixed bg-[#fff] z-[999] justify-center items-center w-full h-[70px]">
      <div className="main-layout">
        <div className="flex items-center justify-between main-container font-[open-sans]">
          <div className="flex items-center min-w-[132px]">
            <div className="cursor-pointer w-10 h-10 md:hidden" onClick={() => {props.toggle()}}>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="100%"
                height="100%"
                fill="#FFD029"
                className="bi bi-list"
                viewBox="0 0 16 16"
              >
                <path
                  fillRule="evenodd"
                  d="M2.5 11.5A.5.5 0 0 1 3 11h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4A.5.5 0 0 1 3 7h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4A.5.5 0 0 1 3 3h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5z"
                ></path>
              </svg>
            </div>
            <img
              src="images/logo.png"
              alt="logo"
              className="w-[55px] h-full my-2 mx-[10px] bg-[#fd6] rounded-[10px]"
            />
            <span className="uppercase text-base lg:text-[22px] xl:text-[27px] xl:leading-[31px] cursor-pointer font-bold text-[#2f2b47]">
              sol kitties
            </span>
          </div>
          <div className="uppercase items-center justify-center hidden md:flex gap-3 px-2">
            <div className="nav-item">verify</div>
            <div className="nav-item">staking</div>
            <div className="nav-item">sweepers</div>
            <div className="nav-item">roadmap</div>
            <div className="nav-item">faq</div>
            <div className="nav-item">whitepaper</div>
          </div>
          <div className="py-[5px]">
            <img
              src="images/join.png"
              alt="join coummunity"
              className="hover:cursor-pointer w-[180px] lg:w-[220px] h-full"
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default Header;

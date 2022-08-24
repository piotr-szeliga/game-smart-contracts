import React, { useState } from "react";
import Footer from "./Footer";
import Header from "./Header";
import Sidebar from "./Sidebar";

type Props = {
  title: string;
  children: React.ReactNode;
};

const MainLayout: React.FC<Props> = ({ title, children }) => {
  const [sideOpen, setSideOpen] = useState(false);

  const close = (arg: boolean): void => {
    setSideOpen(false);
  };

  const toggle = (arg: void): void => {
    setSideOpen(!sideOpen);
  };
  return (
    <div className="relative">
      <Header toggle={toggle} />
      <Sidebar sideOpen={sideOpen} close={close} />
      <div className="pt-[70px]">{children}</div>
      <Footer />
    </div>
  );
};

export default MainLayout;

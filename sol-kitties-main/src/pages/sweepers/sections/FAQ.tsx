import Accordian from "components/Accordian";
import React from "react";

const items = [
  {
    question: "What is Kitties Sweepers?",
    answer:
      "Kitties Sweepers is a special community sweep event for 3D Sol Kitties on Magic Eden.",
  },
  {
    question: "What is Kitties Sweepers?",
    answer:
      "Kitties Sweepers is a special community sweep event for 3D Sol Kitties on Magic Eden.",
  },
  {
    question: "What is Kitties Sweepers?",
    answer:
      "Kitties Sweepers is a special community sweep event for 3D Sol Kitties on Magic Eden.",
  },
  {
    question: "What is Kitties Sweepers?",
    answer:
      "Kitties Sweepers is a special community sweep event for 3D Sol Kitties on Magic Eden.",
  },
  {
    question: "What is Kitties Sweepers?",
    answer:
      "Kitties Sweepers is a special community sweep event for 3D Sol Kitties on Magic Eden.",
  },
];

const FAQ = () => {
  return (
    <div className="main-layout mb-10">
      <div className="main-container">
        <h1 className="faq-title md:mt-10 md:pt-10 text-[#fff] text-[44px] font-black text-center">
          FAQ
        </h1>
        <div className="flex flex-col gap-[17px] mt-10">
          {items.map((item, index) => (
            <Accordian question={item.question} answer={item.answer} key={index} />
          ))}
        </div>
      </div>
    </div>
  );
};

export default FAQ;

import React, { useEffect, useState, type ReactNode } from "react";
import { motion } from "framer-motion";

export const Letter = ({
  content,
  italic,
}: {
  content: string;
  italic: boolean;
}) => {
  const props = italic ? { className: "italic" } : {};
  return <span {...props}>{content}</span>;
};

export const WelcomeTitle = (): ReactNode => {
  const [text, setText] = useState("");
  const printLetter = (text: string): void => {
    const output = "Welcome to Conundrum";
    if (text.length >= output.length) {
      return;
    }
    const contentPlusOne = output.slice(0, text.length + 1);
    setTimeout(() => {
      setText(contentPlusOne);
      printLetter(contentPlusOne);
    }, 50);
  };
  useEffect(() => {
    printLetter("");
  }, []);
  return (
    <motion.h1
      className="font-bold text-4xl lg:text-5xl text-foreground *:text-foreground"
      initial={{
        opacity: 0,
        scale: 0,
      }}
      animate={{
        opacity: 1,
        scale: 1,
      }}
      exit={{
        opacity: 0,
        scale: 0,
      }}
    >
      {text.split("").map((l, i) => {
        return (
          <Letter key={i} content={l} italic={i >= "Welcome to ".length} />
        );
      })}
    </motion.h1>
  );
};

WelcomeTitle.displayName = "WelcomeTitle";

import React, { type ReactNode } from "react";
import { motion } from "framer-motion";
interface PageTitleGroupProps {
  title: ReactNode;
  subtitle?: ReactNode;
}

export const PageTitleGroup = ({
  title,
  subtitle,
}: PageTitleGroupProps): ReactNode => {
  return (
    <>
      <motion.h2
        className="text-2xl font-semibold text-foreground"
        initial={{
          opacity: 0,
        }}
        animate={{
          opacity: 1,
        }}
      >
        {title}
      </motion.h2>
      {subtitle ? (
        <motion.div
          initial={{
            opacity: 0,
          }}
          animate={{
            opacity: 1,
          }}
          transition={{
            delay: 0.1,
          }}
          className="text-foreground/60! text-sm"
        >
          {subtitle}
        </motion.div>
      ) : null}
    </>
  );
};

PageTitleGroup.displayName = "PageTitleGroup";

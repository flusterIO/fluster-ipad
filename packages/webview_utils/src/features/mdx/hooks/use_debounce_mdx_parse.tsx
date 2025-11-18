import React, { useEffect, useState, Fragment, HTMLProps } from "react";
import { run } from "@mdx-js/mdx";
import * as runtime from "react/jsx-runtime";
import * as devRuntime from "react/jsx-dev-runtime";
import type { MDXModule } from "mdx/types";
import { parseMdxString } from "../methods/mdx_to_jsx";
import { ParsedMdxContent } from "../components/parsed_mdx_content";

export const useDebounceMdxParse = (
  initialValue: string = "",
  debounceTimeout: number = 300,
) => {
  const [value, setValue] = useState<string>(initialValue);
  const [hasParsed, setHasParsed] = useState(false);
  const [mdxModule, setMdxModule] = useState<MDXModule | null>(null);
  const [timer, setTimer] = useState<NodeJS.Timeout | null>(null);

  const handleParse = async (_value: string) => {
    try {
      const compiled = await parseMdxString({
        content: _value,
      });
      const res = await run(compiled, {
        Fragment: Fragment,
        jsx: runtime.jsx,
        jsxs: runtime.jsxs,
        jsxDEV: devRuntime.jsxDEV,
        baseUrl: import.meta.url,
      });
      if (res.default.name === "MDXContent") {
        setMdxModule(res);
      } else {
        setMdxModule(null);
      }
    } catch (err) {
      console.warn(`Error: ${err}`);
    }
  };

  const parse = async (_value: string): Promise<void> => {
    try {
      await handleParse(_value);
    } catch (err) {
      console.error("Error: ", err);
    }
    setHasParsed(true);
  };

  useEffect(() => {
    if (timer) {
      clearTimeout(timer);
    }
    if (hasParsed === false) {
      /* eslint-disable-next-line  -- I'll come back to this later. */
      parse(value);
    } else {
      setTimer(setTimeout(() => handleParse(value || ""), debounceTimeout));
    }
    /* eslint-disable-next-line  --  */
  }, [value]);

  const Component = (props: HTMLProps<HTMLDivElement>) =>
    mdxModule ? (
      <ParsedMdxContent
        {...props}
        MdxContentComponent={mdxModule.default}
        raw={value}
      />
    ) : (
      <></>
    );

  return {
    value,
    setValue,
    Component,
    isReady: Boolean(mdxModule),
  };
};

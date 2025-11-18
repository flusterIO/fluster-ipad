import { MDXComponents } from "mdx/types";
import { useMemo } from "react";
import { getComponentMap } from "../methods/get_component_map";

export const useComponentMap = (mdxString: string): MDXComponents => {
    return useMemo(
        () => (mdxString ? getComponentMap(mdxString) : {}),
        [mdxString],
    );
};

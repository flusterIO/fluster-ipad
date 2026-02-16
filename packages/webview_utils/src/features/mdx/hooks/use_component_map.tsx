import { MDXComponents } from "mdx/types";
import { useMemo } from "react";
import { ComponentMapItem, getComponentMap } from "../methods/get_component_map";

export const useComponentMap = (mdxString: string, additionalComponents: ComponentMapItem[] = []): MDXComponents => {
    return useMemo(
        () => (mdxString ? getComponentMap(mdxString, additionalComponents) : {}),
        [mdxString, additionalComponents],
    );
};

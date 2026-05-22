import { type NextjsPageQuery } from "../../../providers/next";
import { type BlogSearchParams } from "../types";

export const blogSearchParamsToQuery = (
    searchParams: Omit<BlogSearchParams, "slug">,
    slug: string[] | null | undefined,
): NextjsPageQuery | undefined => {
    /* eslint-disable-next-line  --  */
    if (!searchParams) {
        return undefined;
    }
    if (Object.keys(searchParams).length === 0) {
        return undefined;
    }
    return {
        ...searchParams,
        slug: slug ?? [],
        tags: {
            anyOf: searchParams.tagAnyOf,
            allOf: searchParams.tagAllOf,
        },
    };
};

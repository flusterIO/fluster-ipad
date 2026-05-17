import { type NextjsPageQuery } from "../../../providers/next";
import { type BlogSearchParams } from "../types";

export const blogSearchParamsToQuery = (
    searchParams: BlogSearchParams,
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
        keywords: {
            anyOf: searchParams.kwAnyOf,
            allOf: searchParams.kwAllOf,
        },
    };
};

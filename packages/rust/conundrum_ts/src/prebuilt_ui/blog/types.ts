import { type GeneralPageQuery } from "../../providers/general/types";

/**
 * Using these is completely optional. Feel free to implement your own search methods, but this makes life easier if you don't already have query strings in place somewhere.
 */
export interface BlogSearchParams extends Omit<GeneralPageQuery, "keywords"> {
    /**
     * Maps to keyword.anyOf
     */
    kwAnyOf?: string[];
    /**
     * Maps to keyword.allOf
     */
    kwAllOf?: string[];
}

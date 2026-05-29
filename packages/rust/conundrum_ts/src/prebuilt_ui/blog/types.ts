import { type FC } from "react";
import { type GeneralPageQuery } from "../../providers/general/types";
import { type WithNullableOptionals } from "../../types/general";
import { type BlogFileSummary } from "../../code_gen";

export type FileData = WithNullableOptionals<BlogFileSummary>;
export type FileSummary = Omit<WithNullableOptionals<BlogFileSummary>, "html">;

export interface BlogCategory {
    label: string;
    /**
     * An `id` is only required if the `label` field occurs or might occur more than once and it must be unique.
     */
    id?: string;
    icon: FC<{ className?: string }>;
}

/**
 * Using these is completely optional. Feel free to implement your own search methods, but this makes life easier if you don't already have query strings in place somewhere.
 */
export interface BlogSearchParams extends Omit<GeneralPageQuery, "keywords"> {
    /**
     * Maps to keyword.anyOf
     */
    tagAnyOf?: string[];
    /**
     * Maps to keyword.allOf
     */
    tagAllOf?: string[];
}

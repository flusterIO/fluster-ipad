import {
    type NextJsConundrumOutput,
    type BlogFileSummary,
} from "../../code_gen/typeshare/conundrum";
import { type GeneralPageQuery } from "../general/types";

export interface NextjsPageQuery extends GeneralPageQuery {
    id?: string;
}

export const getNextJsPages = (
    query: NextjsPageQuery | undefined,
    output: Partial<NextJsConundrumOutput>,
):
    | { results: BlogFileSummary[] | undefined; exactMatch: false }
    | { results: BlogFileSummary; exactMatch: true } => {
    if (!query) {
        return { results: output.files, exactMatch: false };
    }
    if (query.slug?.length) {
        const slugString = query.slug.join("/").toLowerCase();
        const res = output.files?.find((f) => slugString === f.relative_path);
        if (res) {
            return {
                results: res,
                exactMatch: true,
            };
        } else {
            const matchingItems = output.files?.filter((f) =>
                f.relative_path.startsWith(slugString),
            );
            return {
                results: matchingItems,
                exactMatch: false,
            };
        }
    }
    if (query.path) {
        return {
            results: output.files?.filter((f) => {
                return (
                    f.relative_path
                        .toLowerCase()
                        /* eslint-disable-next-line  -- It'll be ok... everything will be ok. */
                        .includes(query.path!.toLowerCase())
                );
            }),
            exactMatch: false,
        };
    }
    if (query.id) {
        return {
            results: output.files?.filter(
                /* eslint-disable-next-line  --  */
                (f) => f.front_matter?.user_defined_id == query.id!,
            ),
            exactMatch: false,
        };
    }
    if (query.subject) {
        return {
            results: output.files?.filter((f) => f.front_matter?.subject === query.subject),
            exactMatch: false
        }
    }
    if (query.topic) {
        return {
            results: output.files?.filter((f) => f.front_matter?.topic === query.topic),
            exactMatch: false
        }
    }
    if (query.tags?.allOf) {
        return {
            results: output.files?.filter((f) =>
                /* eslint-disable-next-line  --  */
                query.tags?.allOf!.map((k) => f.keywords.includes(k)),
            ),
            exactMatch: false,
        };
    }
    if (query.tags?.anyOf) {
        return {
            results: output.files?.filter((f) => {
                /* eslint-disable-next-line  --  */
                for (const k of query.tags!.anyOf!) {
                    if (f.keywords.includes(k)) {
                        return true;
                    }
                }
                return false;
            }),
            exactMatch: false,
        };
    }
    return { results: output.files, exactMatch: false };
};

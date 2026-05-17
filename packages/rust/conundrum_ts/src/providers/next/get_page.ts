import {
    type NextJsConundrumOutput,
    type NextjsFileSummary,
} from "../../code_gen/typeshare/conundrum";
import { type GeneralPageQuery } from "../general/types";

export interface NextjsPageQuery extends GeneralPageQuery {
    id?: string;
}

export const getNextJsPage = (
    query: NextjsPageQuery | undefined,
    output: Partial<NextJsConundrumOutput>,
): NextjsFileSummary[] | undefined => {
    if (!query) {
        return output.files;
    }
    if (query.path) {
        return output.files?.filter((f) => {
            /* eslint-disable-next-line  --  */
            return f.relative_path.toLowerCase().includes(query.path!.toLowerCase());
        });
    }
    if (query.id) {
        return output.files?.filter(
            /* eslint-disable-next-line  --  */
            (f) => f.front_matter?.user_defined_id == query.id!,
        );
    }
    if (query.keywords?.allOf) {
        return output.files?.filter((f) =>
            /* eslint-disable-next-line  --  */
            query.keywords?.allOf!.map((k) => f.keywords.includes(k)),
        );
    }
    if (query.keywords?.anyOf) {
        return output.files?.filter((f) => {
            /* eslint-disable-next-line  --  */
            for (const k of query.keywords!.anyOf!) {
                if (f.keywords.includes(k)) {
                    return true;
                }
            }
            return false;
        });
    }
    return output.files;
};

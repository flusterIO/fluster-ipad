import "@citation-js/plugin-csl";
import "@citation-js/plugin-bibtex";
import Cite from "citation-js";
import { NoteDetailCitationBuffer } from "@/code_gen/flat_buffer/mdx-serialization/note-details";

export interface FormattedCitationData {
    id: string
    idx: number
    html: string
}

export const getFormattedCitations = async (
    content: string,
    items: NoteDetailCitationBuffer[],
    cslFileContent?: string
) => {
    let shouldUseUserDefined = false;
    if (cslFileContent) {
        if (cslFileContent) {
            const config = Cite.plugins.config.get("@csl");
            if (config) {
                shouldUseUserDefined = true;
                config?.templates.add("user-defined", cslFileContent);
            } else {
                console.error("Could not gather csl config. Cannot parse citations with user provided csl format.")
            }
        }
    }
    const citations = new Cite(content);

    const formattedItems: FormattedCitationData[] = []
    for await (const item of items) {
        const id = item.id() as string
        const citationRes = await citations.format("bibliography", {
            format: "html",
            template: shouldUseUserDefined ? "user-defined" : "apa",
            entry: id
        })
        formattedItems.push({
            id,
            idx: item.idx(),
            html: citationRes
        })
    }
    return { citations, formattedItems, userDefined: shouldUseUserDefined };
};

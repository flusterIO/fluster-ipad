import "@citation-js/plugin-csl";
import "@citation-js/plugin-bibtex";
import Cite from "citation-js";

export const getFormattedCitations = (
    content: string,
    cslFileContent?: string
) => {
    let shouldUseUserDefined = false;
    if (cslFileContent) {
        if (cslFileContent) {
            let config = Cite.plugins.config.get("@csl");
            if (config) {
                shouldUseUserDefined = true;
                config?.templates.add("user-defined", cslFileContent);
            } else {
                console.error("Could not gather csl config. Cannot parse citations with user provided csl format.")
            }
        }
    }
    let citations = new Cite(content);
    return { citations, userDefined: shouldUseUserDefined };
};

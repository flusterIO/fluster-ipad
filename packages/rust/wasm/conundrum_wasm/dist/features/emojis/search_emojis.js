import { search_conundrum_emojis, search_conundrum_emojis_in_docs_container, } from "../../../pkg/conundrum_wasm.js";
export const searchConundrumEmojis = (query, page = 1, perPage = 50) => {
    return search_conundrum_emojis(query, page, perPage);
};
export const searchConundrumEmojisAndAppendToContainer = (query, containerId = "emoji-docs-content", page = 1, perPage = 50) => {
    const res = search_conundrum_emojis_in_docs_container(query, containerId, page, perPage);
    const container = document.getElementById(containerId);
    /* eslint-disable-next-line  -- What the fuck are you talking about? Of course it's possibly falsy. */
    if (container) {
        container.innerHTML = res.names.map((item) => item.html).join("");
    }
};
//# sourceMappingURL=search_emojis.js.map
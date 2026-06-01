import initWasm, { search_conundrum_emojis } from "../../pkg/conundrum_wasm.js";
import { searchConundrumEmojisAndAppendToContainer } from "../features/emojis/search_emojis.js";
export const attachConundrumWasm = () => {
    initWasm()
        .then(() => {
        /* eslint-disable-next-line  -- It's not always falsy ya' dumb c--t. */
        if (!window.conundrum) {
            window.conundrum = {};
        }
        window.conundrum.searchEmojis = search_conundrum_emojis;
        window.conundrum.searchConundrumEmojisAndAppendToContainer =
            searchConundrumEmojisAndAppendToContainer;
    })
        .catch((err) => {
        console.error("Error initializing conundrum web-assembly: {}", err);
    });
};
//# sourceMappingURL=attach_to_window.js.map
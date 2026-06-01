import { search_conundrum_emojis } from "../../pkg/conundrum_wasm.js";
import { searchConundrumEmojisAndAppendToContainer } from "../features/emojis/search_emojis.js";
declare global {
    interface Window {
        conundrum?: {
            searchEmojis?: typeof search_conundrum_emojis;
            searchConundrumEmojisAndAppendToContainer?: typeof searchConundrumEmojisAndAppendToContainer;
        };
    }
}
export declare const attachConundrumWasm: () => void;

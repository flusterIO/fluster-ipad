import { search_emojis } from "../../../pkg/fluster_wasm.js";
import { type EmojiData } from "../../core/code_gen/typeshare/conundrum.js";

export const searchConundrumEmojis = (query: string) => {
    return search_emojis(query) as EmojiData[];
};

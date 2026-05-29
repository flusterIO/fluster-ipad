import { search_conundrum_emojis } from "../../../pkg/fluster_wasm.js";
import { type EmojiData } from "../../core/code_gen/typeshare/conundrum.js";

export const searchConundrumEmojis = (
    query: string,
    page: number,
    perPage: number,
) => {
    return search_conundrum_emojis(query, page, perPage) as EmojiData[];
};

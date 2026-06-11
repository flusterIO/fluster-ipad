import { type handleConundrumTabClick } from "./component_glue/tabs/handle_tab_click";
import { type onCopyCodeBlockClick } from "./component_glue/code/on_copy_code";
import { type onCodeBlockContainerClick } from "./component_glue/code/on_codeblock_container_click";
import { type onAdmonitionHeadingClick } from "./component_glue/admonition/on_admonition_heading_click";
import { type onLoad } from "./event_handlers/on_load";
import { type onResize } from "./event_handlers/on_resize";
import { type copyStringToClipboard } from "./utils/copy_string_to_clipboard";
import { type onEmojiDocsInputChange } from "./component_glue/emoji_docs/on_emoji_docs_input_change";
import conundrumWasm from "@conundrum/wasm";
import {
    searchConundrumEmojisAndAppendToContainer,
    searchConundrumEmojis,
    compileConundrum,
} from "@conundrum/wasm/ts";
import { type handleNoteLinkClick } from "./markdown_glue/extensions/note_link/handle_note_link_click";
import { type onDictionaryEntryClick } from "./event_handlers/on_dictionary_entry_click";
import { type onTocItemClick } from "./event_handlers/on_toc_item_click";
import { type handleTagClick } from "./event_handlers/on_tag_click";
import {
    type ConundrumModifier,
    type MdxParsingResult,
    type UIParams,
} from "./code_gen";

export interface ConundrumWebClient {
    handleConundrumTabClick: typeof handleConundrumTabClick;
    handleNoteLinkClick: typeof handleNoteLinkClick;
    onAdmonitionHeadingClick: typeof onAdmonitionHeadingClick;
    onCopyCodeBlockClick: typeof onCopyCodeBlockClick;
    onCodeBlockContainerClick: typeof onCodeBlockContainerClick;
    onResize: typeof onResize;
    onLoad: typeof onLoad;
    copyString: typeof copyStringToClipboard;
    onEmojiDocsInputChange: typeof onEmojiDocsInputChange;
    searchEmojis?: typeof searchConundrumEmojis;
    searchConundrumEmojisAndAppendToContainer?: typeof searchConundrumEmojisAndAppendToContainer;
    onDictionaryEntryClick: typeof onDictionaryEntryClick;
    onTocItemClick: typeof onTocItemClick;
    handleTagClick: typeof handleTagClick;
    compileConundrum?: typeof compileConundrum<
        UIParams,
        ConundrumModifier,
        MdxParsingResult
    >;
}

/**
 * Make sure to call `attachConundrumToWasm` _after_ calling `initializeConundrumWeb`.
 */
export const attachConundrumWasm = (onLoad?: () => void) => {
    conundrumWasm()
        .then(() => {
            /* eslint-disable-next-line  -- It's not always falsy ya' dumb c--t. */
            if (!window.conundrum) {
                window.conundrum = {} as ConundrumWebClient;
            }
            window.conundrum.searchEmojis = searchConundrumEmojis;
            window.conundrum.compileConundrum = compileConundrum;
            window.conundrum.searchConundrumEmojisAndAppendToContainer =
                searchConundrumEmojisAndAppendToContainer;
            if (onLoad) {
                onLoad();
            }
        })
        .catch((err: unknown) => {
            console.error("Error initializing conundrum web-assembly: {}", err);
        });
};

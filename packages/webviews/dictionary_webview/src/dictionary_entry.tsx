import { cn, MdxPreviewWebviewActions, sendToSwift, type GlobalWebviewStateDeepNullable } from "@fluster/webview_utils";
import React, { useEffect, useState, type ReactNode } from "react";
import { compile_cdrm } from "@conundrum/wasm";
import { connect } from "react-redux";
import {
    type MdxParsingResult,
    type UIParams,
    type SupportedCodeBlockTheme,
} from "../../../webview_utils/dist/src/core/code_gen/typeshare/conundrum";

interface DictionaryEntryProps {
    entry: GlobalWebviewStateDeepNullable["dictionary"]["entries"][number];
    darkMode: boolean;
}

interface ParsedDictEntry {
    label: string;
    body: string;
}

const connector = connect((state: GlobalWebviewStateDeepNullable) => ({
    darkMode: state.container.dark_mode,
}));

/**
 *  ## TODO
 *  - [ ] This will not allow varied codeblock themes. Pass them in from Swift and implement them here
 *  - [ ] Internationalization. Make sure things are sorted properly across the language barrier.
 */
export const DictionaryEntry = connector(
    ({ entry, darkMode }: DictionaryEntryProps): ReactNode => {
        const [content, setContent] = useState<null | ParsedDictEntry>(null);
        const [uiParams, setUiParams] = useState<UIParams>({
            dark_mode: true,
            syntax_theme: "Dracula" as SupportedCodeBlockTheme,
            font_scalar: 1,
            math_font_scalar: 1.2,
        });

        useEffect(() => {
            setUiParams({
                ...uiParams,
                dark_mode: darkMode,
            });
        }, [darkMode]);
        const parseData = async (e: typeof entry): Promise<void> => {
            try {
                const resLabel = (await compile_cdrm(
                    e.label,
                    uiParams,
                    true,
                )) as MdxParsingResult;
                const resBody = (await compile_cdrm(
                    e.body,
                    uiParams,
                    true,
                )) as MdxParsingResult;
                setContent({
                    label: resLabel.content,
                    body: resBody.content,
                });
            } catch (err: unknown) {
                console.log("err: ", err)
            }
        };
        useEffect(() => {
            parseData(entry).catch((err: unknown) => {
                console.error("Error: ", err);
            });
        }, [entry]);
        if (!content) {
            return null;
        }
        return (
            <div className="max-w-[1080px] font-serif">
                <h3
                    className={cn("text-lg font-bold", entry.origin_note_id && "cursor-pointer")}
                    dangerouslySetInnerHTML={{ __html: content.label }}
                    onClick={() => {
                        if (entry.origin_note_id) {
                            sendToSwift(MdxPreviewWebviewActions.ViewNoteById, entry.origin_note_id)
                        }
                    }}
                />
                <div className="text-foreground/80 pl-6" dangerouslySetInnerHTML={{ __html: content.body }} />
            </div>
        );
    },
);

DictionaryEntry.displayName = "DictionaryEntry";

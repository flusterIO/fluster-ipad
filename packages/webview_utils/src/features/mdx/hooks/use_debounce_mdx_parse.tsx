import React, { useEffect, useState, Fragment, type HTMLProps } from "react";
import { run } from "@mdx-js/mdx";
import * as runtime from "react/jsx-runtime";
import * as devRuntime from "react/jsx-dev-runtime";
import type { MDXModule } from "mdx/types";
import { parseMdxString } from "../methods/mdx_to_jsx";
import { ParsedMdxContent } from "../components/parsed_mdx_content";
import { type AnyWebviewAction, type AnyWebviewStorageKey } from "@/utils/types/any_window_event";
import { type ComponentMapItem } from "../methods/get_component_map";
import { WebviewClient } from "../../webview_container/data/webview_client";
import { useSelector } from "react-redux";
import { type GlobalWebviewStateNullable } from "#/webview_global_state/cross_language_state_types";
import { type CodeEditorTheme } from "@/code_gen/typeshare/fluster_core_utilities";


declare global {
    interface WindowEventMap {
        "mdx-content-loaded": CustomEvent<{ scollPositionKey: AnyWebviewStorageKey }>;
    }
}

export const useDebounceMdxParse = (
    initialValue = "",
    debounceTimeout = 300,
    /// A unique key that is passed to the mdx-content-loaded event as the detail field if present.
    contentLoadedId: string,
    showWebviewHandler?: AnyWebviewAction,
    additionalComponents?: ComponentMapItem[],
    asMain = false
) => {
    const [value, setValue] = useState<string>(initialValue);
    const [hasParsed, setHasParsed] = useState(false);
    const [mdxModule, setMdxModule] = useState<MDXModule | null>(null);
    const lightCodeTheme = useSelector((state: GlobalWebviewStateNullable) => state.editor.theme_light)
    const darkCodeTheme = useSelector((state: GlobalWebviewStateNullable) => state.editor.theme_dark)
    const mathjaxFontUrl = useSelector((state: GlobalWebviewStateNullable) => state.math.mathjax_font_url)


    const [timer, setTimer] = useState<NodeJS.Timeout | null>(null);

    const handleParse = async (
        _value: string,
        darkCodeTheme: CodeEditorTheme,
        lightCodeTheme: CodeEditorTheme,
        mathjaxFontUrl: string
    ) => {
        try {
            const compiled = await parseMdxString({
                content: _value,
                darkCodeTheme,
                lightCodeTheme,
                mathjaxFontUrl,
            });
            // TODO: DO something with the toc field now that you're grabbing it.
            const res = await run(compiled, {
                Fragment: Fragment,
                jsx: runtime.jsx,
                jsxs: runtime.jsxs,
                jsxDEV: devRuntime.jsxDEV,
                baseUrl: import.meta.url,
            });
            if (res.default.name === "MDXContent") {
                setMdxModule(res);
            } else {
                setMdxModule(null);
            }
        } catch (err) {
            console.warn("Error: ", err);
        }
    };

    const parse = async (
        _value: string,
        darkCodeTheme: CodeEditorTheme,
        lightCodeTheme: CodeEditorTheme,
        mathjaxFontUrl: string
    ): Promise<void> => {
        try {
            await handleParse(_value, darkCodeTheme, lightCodeTheme, mathjaxFontUrl);
            setHasParsed(true);
        } catch (err) {
            console.error("Error: ", err);
            return
        }
    };

    useEffect(() => {
        if (timer) {
            clearTimeout(timer);
        }
        if (!hasParsed) {
            /* eslint-disable-next-line  -- I'll come back to this later. */
            parse(value, darkCodeTheme, lightCodeTheme, mathjaxFontUrl);
        } else {
            setTimer(
                setTimeout(
                    () => {
                        handleParse(value || "", darkCodeTheme, lightCodeTheme, mathjaxFontUrl).catch((err: unknown) => { console.error("Error: ", err); })
                    },
                    debounceTimeout,
                ),
            );
        }

    }, [value, darkCodeTheme, lightCodeTheme, mathjaxFontUrl]);

    const Component = (props: HTMLProps<HTMLDivElement>) =>
        mdxModule ? (
            <ParsedMdxContent
                container={props}
                MdxContentComponent={mdxModule.default}
                raw={value}
                asMain={asMain}
                scrollPositionKey={contentLoadedId}
                showWebviewHandler={showWebviewHandler}
                additionalComponents={additionalComponents}
            />
        ) : (
            <></>
        );

    useEffect(() => {
        WebviewClient.setMdxContentLoaded(contentLoadedId)
    }, [mdxModule])

    return {
        value,
        setValue,
        Component,
        isReady: Boolean(mdxModule && hasParsed),
    };
};

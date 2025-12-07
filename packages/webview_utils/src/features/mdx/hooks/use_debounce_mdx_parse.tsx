import React, { useEffect, useState, Fragment, HTMLProps } from "react";
import { run } from "@mdx-js/mdx";
import * as runtime from "react/jsx-runtime";
import * as devRuntime from "react/jsx-dev-runtime";
import type { MDXModule } from "mdx/types";
import { parseMdxString } from "../methods/mdx_to_jsx";
import { ParsedMdxContent } from "../components/parsed_mdx_content";
import { useLocalStorage } from "@/state/hooks/use_local_storage";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { SplitviewEditorWebviewEvents, SplitviewEditorWebviewLocalStorageKeys } from "@/code_gen/typeshare/fluster_core_utilities";



declare global {

    interface WindowEventMap {
        "mdx-content-loaded": CustomEvent<{ id?: string }>;
    }
}

export const useDebounceMdxParse = (
    initialValue: string = "",
    debounceTimeout: number = 300,
    /// A unique key that is passed to the mdx-content-loaded event as the detail field if present.
    contentLoadedId: string = "mdx-content"
) => {
    const [value, setValue] = useState<string>(initialValue);
    const [hasParsed, setHasParsed] = useState(false);
    const [mdxModule, setMdxModule] = useState<MDXModule | null>(null);
    const [darkCodeTheme, setDarkCodeTheme] = useLocalStorage(
        SplitviewEditorWebviewLocalStorageKeys.CodeThemeDark,
        undefined,
        {
            deserializer(value) {
                return value;
            },
            serializer(value) {
                return value;
            },
            initializeWithValue: false,
        },
    );
    const [lightCodeTheme, setLightCodeTheme] = useLocalStorage(
        SplitviewEditorWebviewLocalStorageKeys.CodeThemeLight,
        undefined,
        {
            deserializer(value) {
                return value;
            },
            serializer(value) {
                return value;
            },
            initializeWithValue: false,
        },
    );

    useEventListener(SplitviewEditorWebviewEvents.SetCodeThemeDark, (e) => setDarkCodeTheme(e.detail));
    useEventListener(SplitviewEditorWebviewEvents.SetCodeThemeLight, (e) => setLightCodeTheme(e.detail));

    const [timer, setTimer] = useState<NodeJS.Timeout | null>(null);

    const handleParse = async (
        _value: string,
        darkCodeTheme: string,
        lightCodeTheme: string,
    ) => {
        try {
            const compiled = await parseMdxString({
                content: _value,
                darkCodeTheme,
                lightCodeTheme,
            });
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
            console.warn(`Error: ${err}`);
        }
    };

    const parse = async (
        _value: string,
        darkCodeTheme: string,
        lightCodeTheme: string,
    ): Promise<void> => {
        try {
            await handleParse(_value, darkCodeTheme, lightCodeTheme);
        } catch (err) {
            console.error("Error: ", err);
        }
        setHasParsed(true);
    };

    useEffect(() => {
        if (timer) {
            clearTimeout(timer);
        }
        if (hasParsed === false) {
            /* eslint-disable-next-line  -- I'll come back to this later. */
            parse(value, darkCodeTheme, lightCodeTheme);
        } else {
            setTimer(
                setTimeout(
                    () => handleParse(value || "", darkCodeTheme, lightCodeTheme),
                    debounceTimeout,
                ),
            );
        }
        /* eslint-disable-next-line  --  */
    }, [value, darkCodeTheme, lightCodeTheme]);

    const Component = (props: HTMLProps<HTMLDivElement>) =>
        mdxModule ? (
            <ParsedMdxContent
                container={props}
                MdxContentComponent={mdxModule.default}
                raw={value}
            />
        ) : (
            <></>
        );

    useEffect(() => {
        window.dispatchEvent(new CustomEvent("mdx-content-loaded", {
            detail: contentLoadedId
        }))
        /* eslint-disable-next-line -- I hate this rule but I'm too lazy to turn it off. */
    }, [mdxModule])

    return {
        value,
        setValue,
        Component,
        isReady: Boolean(mdxModule),
    };
};

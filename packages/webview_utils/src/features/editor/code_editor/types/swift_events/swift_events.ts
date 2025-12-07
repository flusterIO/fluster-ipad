import { MdxParsingResultBuffer } from "@/code_gen/flat_buffer/mdx-serialization";
import { SplitviewEditorWebviewEvents } from "@/code_gen/typeshare/fluster_core_utilities";
import { ByteBuffer } from "flatbuffers";

export interface SwiftEventMap {
    updateEditorValue: string;
}

declare global {
    interface WindowEventMap {
        [SplitviewEditorWebviewEvents.SetSplitviewEditorContent]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetParsedMdxContent]: CustomEvent<MdxParsingResultBuffer>;
        [SplitviewEditorWebviewEvents.SetParsedMdxContentString]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetEditorKeymap]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetCodeTheme]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetCodeThemeLight]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetCodeThemeDark]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.ResetPreviewScrollPosition]: CustomEvent<null>;
    }
    interface Window {
        setEditorContent: typeof setEditorContent;
        setParsedEditorContent: typeof setParsedEditorContent;
        setParsedEditorContentString: typeof setParsedEditorContentString;
        setEditorKeymap: typeof setEditorKeymap;
        setCodeSyntaxTheme: typeof setCodeTheme;
        setCodeSyntaxThemeLight: typeof setCodeThemeLight;
        setCodeSyntaxThemeDark: typeof setCodeThemeDark;
        resetMdxPreviewScrollPosition: typeof resetMdxPreviewScrollPosition;
    }
}

export function setEditorContent(payload: string) {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetSplitviewEditorContent, { detail: payload }),
    );
}

export function setParsedEditorContent(payload: Uint8Array) {
    console.log("payload: ", payload)
    console.log("payload.length: ", payload.length)
    const data = Uint8Array.from(payload)
    const buf = new ByteBuffer(data)
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetParsedMdxContent, { detail: MdxParsingResultBuffer.getRootAsMdxParsingResultBuffer(buf) }),
    );
}

export function setParsedEditorContentString(payload: string) {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetParsedMdxContentString, { detail: payload }),
    );
}

export function setEditorKeymap(keymap: string) {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetEditorKeymap, { detail: keymap }),
    );
}

export function setCodeTheme(theme: string) {
    window.dispatchEvent(new CustomEvent(SplitviewEditorWebviewEvents.SetCodeTheme, { detail: theme }));
}

export function setCodeThemeDark(theme: string) {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetCodeThemeDark, { detail: theme }),
    );
}

export function setCodeThemeLight(theme: string) {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetCodeThemeLight, { detail: theme }),
    );
}

const resetMdxPreviewScrollPosition = (): void => {
    console.info(`Resetting scroll position`)
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.ResetPreviewScrollPosition, {
            detail: null,
        }),
    );
};

export const setWindowBridgeFunctions = () => {
    window.setEditorContent = setEditorContent;
    window.setEditorKeymap = setEditorKeymap;
    window.setParsedEditorContent = setParsedEditorContent;
    window.setParsedEditorContentString = setParsedEditorContentString;
    window.setCodeSyntaxTheme = setCodeTheme;
    window.setCodeSyntaxThemeDark = setCodeThemeDark;
    window.setCodeSyntaxThemeLight = setCodeThemeLight;
    window.resetMdxPreviewScrollPosition = resetMdxPreviewScrollPosition;
};

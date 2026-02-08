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
        [SplitviewEditorWebviewEvents.EmitMdxParsingError]: CustomEvent<null>
        [SplitviewEditorWebviewEvents.EmitMdxParsingSuccess]: CustomEvent<null>
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
        emitMdxParsingError: typeof emitMdxParsingError
        emitMdxParsingSuccess: typeof emitMdxParsingSuccess
    }
}

export function setEditorContent(payload: string) {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetSplitviewEditorContent, { detail: payload }),
    );
}

export function setParsedEditorContent(payload: Uint8Array) {
    const data = Uint8Array.from(payload)
    const buf = new ByteBuffer(data)
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetParsedMdxContent, { detail: MdxParsingResultBuffer.getRootAsMdxParsingResultBuffer(buf) }),
    );
}

export function setParsedEditorContentString(payload: string) {
    document.body.classList.add("loading")
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetParsedMdxContentString, { detail: payload }),
    );
}

export function setEditorKeymap(keymap: string) {
    console.log("keymap: ", keymap)
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

const resetMdxPreviewScrollPosition = (containerId?: string, scrollPositionKeys?: string[]): void => {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.ResetPreviewScrollPosition, {
            detail: null,
        }),
    );
    if (scrollPositionKeys) {
        for (const k of scrollPositionKeys) {
            window.localStorage.removeItem(k)
        }
    }
    if (containerId) {
        const em = document.getElementById(containerId)
        if (em) {
            em.scrollTop = 0
        }
    }
};

const emitMdxParsingError = (): void => {
    window.dispatchEvent(new CustomEvent(SplitviewEditorWebviewEvents.EmitMdxParsingError, {
        detail: null
    }))
}


const emitMdxParsingSuccess = (): void => {
    window.dispatchEvent(new CustomEvent(SplitviewEditorWebviewEvents.EmitMdxParsingSuccess, {
        detail: null
    }))
}

export const setMdxBridgeFunctions = () => {
    window.setCodeSyntaxTheme = setCodeTheme;
    window.setCodeSyntaxThemeDark = setCodeThemeDark;
    window.setCodeSyntaxThemeLight = setCodeThemeLight;
}

export const setWindowBridgeFunctions = () => {
    window.setEditorContent = setEditorContent;
    window.setEditorKeymap = setEditorKeymap;
    window.setParsedEditorContent = setParsedEditorContent;
    window.setParsedEditorContentString = setParsedEditorContentString;
    window.resetMdxPreviewScrollPosition = resetMdxPreviewScrollPosition;
    window.emitMdxParsingError = emitMdxParsingError;
    window.emitMdxParsingSuccess = emitMdxParsingSuccess;
    setMdxBridgeFunctions()
};

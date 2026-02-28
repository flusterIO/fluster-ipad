import { isWebviewOfEnv } from "#/mdx/components/editor_dom_utils";
import { SerializedString } from "@/code_gen/flat_buffer/shared-webview-data";
import { GetSnippetPropsBuffer } from "@/code_gen/flat_buffer/snippets";
import { SharedWebviewActions, SharedWebviewEvents, SplitviewEditorWebviewEvents, WebviewEnvironment } from "@/code_gen/typeshare/fluster_core_utilities";
import { ByteBuffer } from "flatbuffers";

export interface SwiftEventMap {
    updateEditorValue: string;
}

declare global {
    interface WindowEventMap {
        [SplitviewEditorWebviewEvents.SetSplitviewEditorContent]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetEditorSnippetProps]: CustomEvent<GetSnippetPropsBuffer>;
        [SplitviewEditorWebviewEvents.SetParsedMdxContentString]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetEditorKeymap]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetCodeTheme]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetCodeThemeLight]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetCodeThemeDark]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.ResetPreviewScrollPosition]: CustomEvent<null>;
        [SplitviewEditorWebviewEvents.EmitMdxParsingError]: CustomEvent<null>
        [SplitviewEditorWebviewEvents.EmitMdxParsingSuccess]: CustomEvent<null>
        [SharedWebviewEvents.LocalStorageWrite]: CustomEvent<null>
    }
    interface Window {
        setEditorContent: typeof setEditorContent;
        setParsedEditorContentString: typeof setParsedEditorContentString;
        setEditorKeymap: typeof setEditorKeymap;
        setCodeSyntaxTheme: typeof setCodeTheme;
        setCodeSyntaxThemeLight: typeof setCodeThemeLight;
        setCodeSyntaxThemeDark: typeof setCodeThemeDark;
        resetMdxPreviewScrollPosition: typeof resetMdxPreviewScrollPosition;
        emitMdxParsingError: typeof emitMdxParsingError
        emitMdxParsingSuccess: typeof emitMdxParsingSuccess
        setSnippetProps: typeof setParsedEditorSnippetProps;
    }
}


export function setEditorContent(payload: string) {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetSplitviewEditorContent, { detail: payload }),
    );
}

export function setParsedEditorSnippetProps(payload: Uint8Array) {
    const data = Uint8Array.from(payload)
    const buf = new ByteBuffer(data)
    window.dispatchEvent(
        new CustomEvent(
            SplitviewEditorWebviewEvents.SetEditorSnippetProps,
            {
                detail: GetSnippetPropsBuffer.getRootAsGetSnippetPropsBuffer(buf)
            }
        ),
    );
}


// TODO: Now that strings are serializing properly as f--king strings, go back to just using a string here for simplicity's sake.
export function setParsedEditorContentString(payload: Uint8Array) {
    if (isWebviewOfEnv(WebviewEnvironment.IPad)) {
        document.body.classList.add("loading")
    }

    const data = Uint8Array.from(payload)
    const buf = new ByteBuffer(data)
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetParsedMdxContentString, { detail: SerializedString.getRootAsSerializedString(buf).body() ?? "" }),
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
    window.setSnippetProps = setParsedEditorSnippetProps;
    window.setParsedEditorContentString = setParsedEditorContentString;
    window.resetMdxPreviewScrollPosition = resetMdxPreviewScrollPosition;
    window.emitMdxParsingError = emitMdxParsingError;
    window.emitMdxParsingSuccess = emitMdxParsingSuccess;
    setMdxBridgeFunctions()
};

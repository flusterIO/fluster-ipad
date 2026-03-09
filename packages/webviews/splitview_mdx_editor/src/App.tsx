import React from "react";
import {
    ResponsiveSplitViewEditor,
    MdxEditorGlobalProvider,
    WebViewContainer,
    type EditorStateActions,
    type AnyCrossLanguageEditorAction,
    createFlusterStore,
    handleSwiftBufferAction,
    handleSwiftAction,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";

const storeData = createFlusterStore();

declare global {
    interface Window {
        handleEditorStateParsedContentUpdate: (data: number[]) => void;
        handleSwiftAction: typeof handleSwiftActionWrapper;
        handleSwiftBufferAction: typeof handleSwiftBufferActionWrapper;
    }
}

const handleSwiftActionWrapper = (
    action: AnyCrossLanguageEditorAction,
): void => {
    storeData.store.dispatch(handleSwiftAction(action));
};

window.handleSwiftAction = handleSwiftActionWrapper;

const handleSwiftBufferActionWrapper = (
    actionKey: EditorStateActions,
    buf: number[],
): void => {
    storeData.store.dispatch(
        handleSwiftBufferAction({
            type: actionKey,
            payload: buf,
        }),
    );
};

window.handleSwiftBufferAction = handleSwiftBufferActionWrapper;

function App() {
    return (
        <WebViewContainer contentContainerClasses="h-full">
            <MdxEditorGlobalProvider
                store={storeData.store}
                persistor={storeData.persistor}
            >
                <ResponsiveSplitViewEditor />
            </MdxEditorGlobalProvider>
        </WebViewContainer>
    );
}

export default App;

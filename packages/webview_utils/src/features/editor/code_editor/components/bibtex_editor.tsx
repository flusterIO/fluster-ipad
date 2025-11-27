import React, { useEffect, type ReactNode } from "react";
import { CodeEditorInner } from "./code_editor";
import { bibtex } from "@citedrive/codemirror-lang-bibtex";
import {
    CodeEditorProvider,
    useCodeEditorContext,
    useCodeEditorDispatch,
} from "../state/code_editor_provider";
import { useLocalStorage } from "@/state/hooks/use_local_storage";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { LoadingComponent } from "@/shared_components/loading_component";
import { sendToSwift, SwiftHandler } from "@/utils/bridge/send_to_swift";
import { setBibtexEditorWindowBridgeFunctions } from "../types/swift_events/bibtex_editor_swift_events";

setBibtexEditorWindowBridgeFunctions();

const BibtexEditorInner = (): ReactNode => {
    const data = useCodeEditorContext();
    const dispatch = useCodeEditorDispatch();
    const [initialValue, setInitialValue] = useLocalStorage(
        "bibtex-editor-initial-value",
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
    useEventListener("set-bibtex-editor-content", (e) => {
        setInitialValue(e.detail);
        dispatch({
            type: "setInitialEditorValue",
            payload: e.detail,
        });
    });

    useEffect(() => {
        if (!data.haveSetInitialValue) {
            sendToSwift(SwiftHandler.bibtexRequestInitialData, "");
        }
    }, [data.haveSetInitialValue]);
    return data.haveSetInitialValue ? (
        <CodeEditorInner
            initialValue={initialValue}
            language={bibtex()}
            updateHandler={SwiftHandler.bibtexEditorUpdate}
        />
    ) : (
        <div className="w-full h-full flex flex-col justify-center items-center">
            <LoadingComponent />
        </div>
    );
};

export const BibtexEditor = (): ReactNode => {
    return (
        <CodeEditorProvider initialValueKey="bibtex-editor-initial-value">
            <BibtexEditorInner />
        </CodeEditorProvider>
    );
};

BibtexEditor.displayName = "BibtexEditor";

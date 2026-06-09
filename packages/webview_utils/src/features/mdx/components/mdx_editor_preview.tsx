import React, { type HTMLProps, useRef, type ReactNode, useEffect } from "react";
import { cn } from "@/utils/cn";
import { type EditorState, EditorView, EquationNumberingStrategy, SplitviewEditorDomIds, SplitviewEditorWebviewActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { ErrorBoundary } from "react-error-boundary";
import { PreviewLevelErrorReport } from "../error_reporting/preview_level_error_report/preview_level_error_report";
import { type GlobalAppState } from "#/webview_global_state/store";
import { connect, useSelector, useStore } from "react-redux";
import consola from "consola";
import { type WithNullableOptionals } from "../../../core/utils/types/utility_types";
import { ConundrumErrorListener } from "./conundrum_error_listener";
import { NerdFontLoader } from "#/code/nerd_font_loader";
import { KatexFontLoader } from "#/math/katex_font_loader";
import { LucideFontLoader } from "#/themeing/lucide_font_loader";
import { type GlobalWebviewStateDeepNullable } from "#/webview_global_state/cross_language_state_types";
import { ConundrumListener } from "#/webview_global_state/notification_state/conundrum_notification_listener";
import { BibliographyEntryComponent } from "../../bibliography/presentation/bibliography_entry";
import { H3 } from "../../../core/shared_components/typography/typography";


export type MdxEditorPreviewProps = Omit<HTMLProps<HTMLDivElement>, "ref" | "id" | "value">

const connector = connect((state: GlobalAppState) => ({
    lockEditorScrollToPreview: state.editor.lockEditorScrollToPreview,
    isEditorView: state.editor.editorView === EditorView.Splitview,
    hideEquationLabels: state.math.hide_equation_labels,
    citations: state.editor.citations
}))



/**
 * For use _only_ in the primary mdx preview views, either the standalone-preview webview
 * or the splitview editor. 
 * **Do NOT** use this for anything else, as certain state will be inconsistent.
 */
export const MdxEditorPreview = connector(({
    className,
    hideEquationLabels,
    /* lockEditorScrollToPreview, */
    isEditorView,
    citations,
    /* ...props */
}: MdxEditorPreviewProps & Pick<WithNullableOptionals<EditorState>, "lockEditorScrollToPreview"> & { isEditorView: boolean, hideEquationLabels: EquationNumberingStrategy, citations: GlobalAppState["editor"]["citations"] }): ReactNode => {
    const ref = useRef<null | HTMLDivElement>(null)

    const parsedValueIsEmpty = useSelector((state: GlobalWebviewStateDeepNullable) => {
        const x = state.editor.parsedValue;
        if (!x) {
            return false
        }
        return x.trim().length === 0
    })


    useEventListener("set-editor-scroll-proportion", (e) => {
        // This event will only be sent when the editor is in view,
        // meaning this will never run in portrait or in preview mode.
        if (!ref.current) {
            return
        }
        const newProp = e.detail * (ref.current.scrollHeight - ref.current.clientHeight)

        ref.current.scrollTop = newProp
    })

    const store = useStore<GlobalWebviewStateDeepNullable>();


    useEffect(() => {
        /// Set the content from the state initiailly, but do not pas any dependencies to avoid the react render cycle, and use direct html injection.
        const state = store.getState();
        let observer: MutationObserver | null = null;
        if (ref.current) {
            if (state.editor.parsedValue) {
                ref.current.innerHTML = state.editor.parsedValue;
            }
            observer = new MutationObserver(() => {
                window.conundrum?.onLoad()
            })

            observer.observe(ref.current, {
                childList: true,
                subtree: true,
            })
        }
        window.conundrum?.onLoad();
        return () => {
            if (observer) {
                observer.disconnect()
            }
        }
    }, [])



    /* if (typeof parsedValue !== "string") { */
    /*     return ( */
    /*         <div className="w-screen h-screen min-h-screen top-0 left-0 right-0 bottom-0 fixed flex flex-col justify-center items-center mdx-preview-loading-container"> */
    /*             <LoadingComponent /> */
    /*         </div> */
    /*     ); */
    /* } */

    if (parsedValueIsEmpty) {
        return (
            <div className="w-full h-full flex flex-col justify-center items-center note-content-empty-container">
                <div
                    className="text-xl font-semibold"
                    style={{
                        color: "hsl(var(--foreground))",
                    }}
                >
                    Content Empty
                </div>
            </div>
        );
    }


    return (
        <ErrorBoundary
            onError={(e) => { consola.error("Preview boundary error: ", e); }}
            FallbackComponent={(p) => <PreviewLevelErrorReport {...p} showWebviewAction={SplitviewEditorWebviewActions.SetWebviewLoaded} />}
        >
            <ConundrumErrorListener />
            <ConundrumListener />
            <NerdFontLoader />
            <KatexFontLoader />
            <LucideFontLoader />
            <div
                className={cn("",
                    "max-w-[1080px]",
                    isEditorView ? "px-6 pt-4 pb-16 mx-auto max-h-full overflow-y-auto" : "px-8 pt-6 max-h-screen overflow-y-auto pb-16",
                )}
            >

                <div
                    id={SplitviewEditorDomIds.MdxPreview}
                    className={cn(
                        className,
                        isEditorView && "overflow-y-hidden",
                        hideEquationLabels === EquationNumberingStrategy.None ? "hide-equation-labels" : hideEquationLabels === EquationNumberingStrategy.IdOnly ? "hide-no-id-eq-labels" : ""
                    )}
                    ref={ref}
                />
                {citations.length ? (
                    <div className="w-full max-w-full">
                        <H3>Bibliography</H3>
                        <div>
                            {citations.map((cit) => <BibliographyEntryComponent key={cit.citation_key} entry={cit} />)}
                        </div>
                    </div>
                ) : null}
            </div>
        </ErrorBoundary>
    );
});

MdxEditorPreview.displayName = "MdxEditorPreview";

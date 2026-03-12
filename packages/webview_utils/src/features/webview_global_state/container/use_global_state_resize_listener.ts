import { type ReactNode, useEffect } from "react";
import { useMediaQuery } from "react-responsive";
import { setEditorView } from "../mdx_editor/state/editor_state_slice";
import { EditorView, SizableOption, SplitviewEditorDomIds, type WebviewContainerState } from "@/code_gen/typeshare/fluster_core_utilities";
import { getSmallestSizableBreakpointByWidth } from "#/mdx/embeddable_mdx_components/grid/embeddable_responsive_grid_props";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { setSize } from "./webview_container_global_state/webview_container_slice";


import { connect, useDispatch } from 'react-redux';
import { type GlobalAppState } from '#/webview_global_state/store';
const connector = connect((state: GlobalAppState) => ({
    size: state.container.size
}))

export const GlobalStateListeners = connector(({ size }: {
    size: WebviewContainerState["size"]
}): ReactNode => {
    const dispatch = useDispatch()


    // -- Size --
    const onResize = (): void => {
        const em = document.getElementById(SplitviewEditorDomIds.MdxPreview)
        if (!em) {
            console.warn("Could not find mdx-preview container")
            return
        } else {
            const width = em.getBoundingClientRect().width
            const smallestSize = getSmallestSizableBreakpointByWidth(width) ?? SizableOption.Full
            if (smallestSize !== size) {
                dispatch(setSize(smallestSize))
            }
        }
    }
    useEventListener("main-panel-resize", () => {
        onResize()
    })
    useEventListener("redux-state-loaded", () => {
        onResize()
    })
    useEffect(() => {
        onResize()
        window.addEventListener('resize', onResize)
        return () => {
            window.removeEventListener("resize", onResize)
        }
    }, [])

    const isLandscape = useMediaQuery({
        orientation: "landscape",
    });

    useEffect(() => {
        dispatch(setEditorView(isLandscape ? EditorView.Splitview : EditorView.PreviewOnly))
    }, [isLandscape])
    return null
})

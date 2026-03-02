import React, { useEffect, type ReactNode } from 'react'
import { useWebviewContainerContext, useWebviewContainerDispatch } from '../state/webview_provider'
import { getSmallestSizableBreakpointByWidth } from '#/mdx/embeddable_mdx_components/grid/embeddable_responsive_grid_props'
import { useEventListener } from '@/state/hooks/use_event_listener'
import { SplitviewEditorDomIds } from '../../../core/code_gen/typeshare/fluster_core_utilities'


export const WebviewContainerSizableObserver = (): ReactNode => {
    const state = useWebviewContainerContext()
    const dispatch = useWebviewContainerDispatch()
    const onResize = (): void => {
        const em = document.getElementById(SplitviewEditorDomIds.MdxPreview)
        if (!em) {
            console.warn("Could not find mdx-preview container")
            return
        } else {
            const width = em.getBoundingClientRect().width
            const smallestSize = getSmallestSizableBreakpointByWidth(width) ?? "full"
            if (smallestSize !== state.size) {
                dispatch({
                    type: "set-webview-size",
                    payload: smallestSize
                })
            }
        }
    }
    useEventListener("main-panel-resize", () => {
        onResize()
    })
    useEffect(() => {
        onResize()
        window.addEventListener('resize', onResize)
        return () => window.removeEventListener("resize", onResize)
    }, [])
    return null
}


WebviewContainerSizableObserver.displayName = "WebviewContainerSizableObserver"

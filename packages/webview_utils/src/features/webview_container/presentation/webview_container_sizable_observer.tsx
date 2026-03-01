import React, { RefObject, useEffect, type ReactNode } from 'react'
import { useWebviewContainerContext, useWebviewContainerDispatch } from '../state/webview_provider'
import { getSmallestSizableBreakpointByWidth } from '#/mdx/embeddable_mdx_components/grid/embeddable_responsive_grid_props'
import { useEventListener } from '@/state/hooks/use_event_listener'



interface WebviewContainerSizableObserverProps {
    ref: RefObject<HTMLDivElement | null>
}

export const WebviewContainerSizableObserver = ({ ref }: WebviewContainerSizableObserverProps): ReactNode => {
    const state = useWebviewContainerContext()
    const dispatch = useWebviewContainerDispatch()
    const onResize = (): void => {
        if (ref.current === null) {
            return
        } else {
            const smallestSize = getSmallestSizableBreakpointByWidth(ref.current?.getBoundingClientRect().width ?? 0) ?? "full"
            if (smallestSize !== state.size) {
                dispatch({
                    type: "set-webview-size",
                    payload: smallestSize
                })
            }
        }
    }
    useEventListener("main-panel-resize", onResize)
    useEffect(() => {
        onResize()
        window.addEventListener('resize', onResize)
        return () => window.removeEventListener("resize", onResize)
    }, [])
    return null
}


WebviewContainerSizableObserver.displayName = "WebviewContainerSizableObserver"

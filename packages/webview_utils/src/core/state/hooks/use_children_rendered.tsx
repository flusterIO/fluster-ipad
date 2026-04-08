import React, { Children, type FC, useEffect, useRef, type ReactNode, useLayoutEffect, HTMLElementType } from 'react'



interface ChildRenderedEventProps {
    /// A generic id that can be used on a per-usecase basis to narrow down the components that are emitting the event.
    id: string
}

declare global {

    interface WindowEventMap {
        "child-rendered": CustomEvent<ChildRenderedEventProps>;
    }
}



export const useChildrenRendered = <T extends HTMLElement>(callBack: (em: T | undefined | null) => void, debounce = 50) => {
    const debounceTimer = useRef<NodeJS.Timeout | null>(null);

    const ref = useRef<T>(null)
    const handleMutation: MutationCallback = () => {
        if (debounceTimer.current) {
            clearTimeout(debounceTimer.current)
        }
        debounceTimer.current = setTimeout(() => {
            console.log(`Calling back...`)
            callBack(ref.current)
        }, debounce)
    }
    useLayoutEffect(() => {
        const observer = new MutationObserver(handleMutation)
        const em = ref.current;
        if (em) {
            observer.observe(em, {
                childList: true,
                subtree: true
            })
        }
    }, [])
    return ref
}


export const ChildRenderWrapper = ({ children, id }: { children: ReactNode, id: string }): ReactNode => {
    useEffect(() => {
        window.dispatchEvent(new CustomEvent("child-rendered", {
            detail: {
                id
            }
        }))
    }, [])
    return (
        <>
            {children}
        </>
    )
    }

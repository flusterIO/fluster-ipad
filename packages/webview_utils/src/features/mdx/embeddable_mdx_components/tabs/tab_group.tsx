import React, { Children, useEffect, useLayoutEffect, useRef, useState, type ReactNode } from 'react'
import { type EmbeddableTabGroupPropsOutput, tabGroupComponentProps, type EmbeddableTabGroupProps } from './tab_group_props'
import { EmbeddableTabGroupProvider, useEmbeddableTabGroupContext } from './state/embeddable_tab_group_context';
import { type WithChildren } from '../../../../core/utils/types/utility_types';
import { cn } from '@/utils/cn';
import { EmbeddableTabGroupTopBanner } from './tab_group_top_banner';
import { useEventListener } from '@/state/hooks/use_event_listener';
import { v4 } from "uuid"
import { motion } from "framer-motion"
import { ChildRenderWrapper, useChildrenRendered } from '@/state/hooks/use_children_rendered';



const MAX_HEIGHT = 450


export const EmbeddableTabGroup = ({ children, ...props }: EmbeddableTabGroupPropsOutput & WithChildren): ReactNode => {
    const { focusedIndex, tabGroupId } = useEmbeddableTabGroupContext()
    const [height, setHeight] = useState(0)
    const totalChildren = Children.count(children)
    const renderedChildren = useRef(0);
    const heightOfFocusedIndex = (): number | undefined => {
        const em = document.getElementById(`${tabGroupId}-${focusedIndex}`);
        if (em) {
            return Math.min(em.getBoundingClientRect().height, MAX_HEIGHT)
        } else {
            return
        }
    }
    const ref = useChildrenRendered<HTMLDivElement>((em) => {
        const items = em?.querySelectorAll(".tab-group-tab")
        if (items) {
            console.log("heightOfFocusedIndex(): ", heightOfFocusedIndex())
            setHeight(heightOfFocusedIndex() ?? Math.min(Math.max(...items.entries().map((n) => n[1].getBoundingClientRect().height)), MAX_HEIGHT))
        } else {
            setHeight(heightOfFocusedIndex() ?? MAX_HEIGHT)
        }
    })
    const handleHeight = () => {
        console.log(`Handling height?`)
        const items = ref.current?.querySelectorAll(".tab-group-tab")
        if (items) {
            console.log("heightOfFocusedIndex(): ", heightOfFocusedIndex())
            setHeight(heightOfFocusedIndex() ?? Math.min(Math.max(...items.entries().map((n) => n[1].getBoundingClientRect().height)), MAX_HEIGHT))
        } else {
            setHeight(heightOfFocusedIndex() ?? MAX_HEIGHT)
        }
    }

    useEffect(() => {
        if (renderedChildren.current >= totalChildren) {
            handleHeight()
        }
    }, [focusedIndex])

    useEffect(() => {
        window.addEventListener("resize", handleHeight)
        return () => { window.removeEventListener("resize", handleHeight); }
    }, [])

    useEventListener("main-panel-resize", () => {
        console.log(`Here?`)
        handleHeight()
    })

    useLayoutEffect(() => {
        window.addEventListener("child-rendered", (e) => {
            if (e.detail.id === tabGroupId) {
                renderedChildren.current += 1
            }
        })
    }, [])
    return (
        <div ref={ref} className={cn("w-full max-w-[1080px] h-fit", props.containerClasses)}>
            <EmbeddableTabGroupTopBanner />
            <motion.div className="w-full bg-fd-card rounded-br rounded-bl relative overflow-x-hidden overflow-y-auto"
                animate={{
                    height
                }}
            >
                {Children.map(children, (child) => {
                    return (
                        <ChildRenderWrapper id={tabGroupId}>{child}</ChildRenderWrapper>
                    )
                })}
            </motion.div>
        </div>
    )
}


export const WrappedEmbeddableTabGroup = ({ children, ..._props }: EmbeddableTabGroupProps & WithChildren) => {
    const props = tabGroupComponentProps.parse(_props)
    return (
        <EmbeddableTabGroupProvider
            initialValues={{
                activeTabClasses: props.buttonClasses,
                tabGroupId: v4(),
                subtle: props.subtle ?? false
            }}
        >
            <EmbeddableTabGroup {...props}>
                {children}
            </EmbeddableTabGroup>
        </EmbeddableTabGroupProvider>
    )
}


EmbeddableTabGroup.displayName = "EmbeddableTabGroup"

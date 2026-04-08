import React, { type FC, useEffect, type ReactNode, type ReactElement } from 'react'
import { embeddableTabProps, type EmbeddableTabItemProps } from './embeddable_tab_props'
import { type WithChildren } from '../../../../core/utils/types/utility_types'
import { useEmbeddableTabGroupContext, useEmbeddableTabGroupDispatch } from './state/embeddable_tab_group_context'
import { type EmbeddableTabItem } from './types'
import { motion } from 'framer-motion'


export const EmbeddableTab = ({ children, ..._props }: EmbeddableTabItemProps & WithChildren): ReactNode => {
    const { tabs, focusedIndex, tabGroupId } = useEmbeddableTabGroupContext()
    const dispatch = useEmbeddableTabGroupDispatch()
    const props = embeddableTabProps.transform((t) => {
        const item: EmbeddableTabItem = {
            label: (t.label as ReactElement | undefined) ?? t.labelString,
            id: t.id ?? t.labelString
        }
        return {
            item
        }
    }).parse(_props);

    const index = tabs.findIndex((n) => n.id == props.item.id)
    useEffect(() => {
        dispatch({
            type: "addTab",
            payload: props.item
        })
    }, [])

    return (
        <motion.div
            className={"tab-group-tab w-full absolute px-4 py-3"}
            id={`${tabGroupId}-${index}`}
            animate={{
                x: `${(focusedIndex - index) * 100}%`,
                opacity: focusedIndex === index ? 1 : 0
            }}
            transition={{
                bounce: 0.1
            }}
        >
            {children}
        </motion.div>
    )
}


EmbeddableTab.displayName = "EmbeddableTab"

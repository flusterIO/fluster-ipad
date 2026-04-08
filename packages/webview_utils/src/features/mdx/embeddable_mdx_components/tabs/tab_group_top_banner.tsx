import React, { type ReactNode } from 'react'
import { TabGroupButton } from './tab_group_button'
import { useEmbeddableTabGroupContext } from './state/embeddable_tab_group_context'


export const EmbeddableTabGroupTopBanner = (): ReactNode => {
    const { tabs } = useEmbeddableTabGroupContext()
    return (
        <div className="w-full flex flex-row justify-start items-start flex-nowrap overflow-x-auto overflow-y-none">
            {tabs.map((item, idx) => {
                return (
                    <TabGroupButton idx={idx} isLast={idx === tabs.length - 1} item={item} key={item.id} />
                )
            })}
        </div>
    )
}


EmbeddableTabGroupTopBanner.displayName = "EmbeddableTabGroupTopBanner"

import { WrappedEmbeddableTabGroup } from '#/mdx/embeddable_mdx_components/tabs/tab_group'
import { EmbeddableTab } from "#/mdx/embeddable_mdx_components/tabs/tab_group_tab"
import React, { type ReactNode } from 'react'
import { faker } from "@faker-js/faker"

const MAX_PARAGRAPHS = 10

const FakeTab = (): ReactNode => {
    const s = faker.lorem.words({ min: 1, max: 3 })
    return (
        <EmbeddableTab label={s} labelString={s}>
            {faker.lorem.paragraphs({ min: 1, max: MAX_PARAGRAPHS }).replaceAll("\n", "\n\n")}
        </EmbeddableTab>
    )
}


export const TabGroupDevWrapper = (): ReactNode => {
    return (
        <WrappedEmbeddableTabGroup primary subtle>
            <FakeTab />
            <FakeTab />
            <FakeTab />
            <FakeTab />
            <FakeTab />
            <FakeTab />
        </WrappedEmbeddableTabGroup>
    )
}


TabGroupDevWrapper.displayName = "TabGroupDevWrapper"

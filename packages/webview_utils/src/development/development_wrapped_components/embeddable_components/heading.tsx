import { AutoInsertedHeading } from '#/mdx/embeddable_mdx_components/auto_inserted/heading'
import React, { type ReactNode } from 'react'
import { faker } from "@faker-js/faker"



interface HeadingDevelopmentWrapperProps {
    depth?: number
}

export const HeadingDevelopmentWrapper = ({ depth = faker.helpers.arrayElement([1, 2, 3, 4, 5, 6]) }: HeadingDevelopmentWrapperProps): ReactNode => {
    return (
        <AutoInsertedHeading depth={depth as 1} id={null} subtitle={<p>My subtitle!</p>}>My auto inserted heading!</AutoInsertedHeading>
    )
}


HeadingDevelopmentWrapper.displayName = "HeadingDevelopmentWrapper"
